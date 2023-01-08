#![allow(clippy::missing_safety_doc)]

extern crate libc;
use anyhow::Result;
use config::{Config, Environment, File};
use gateway::GatewayServer;
use leash::Component;
use opentelemetry::{
    global,
    sdk::{propagation::TraceContextPropagator, trace, Resource},
    KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use ratelimit::RatelimiterServerComponent;
use rest::ReverseProxyServer;
use serde::de::DeserializeOwned;
use serde_json::Value;
use shared::config::Settings;
use std::{
    env,
    ffi::{CStr, CString},
    str::FromStr,
    time::Duration,
};
use tokio::{
    runtime::Runtime,
    sync::oneshot::{self, Sender},
    task::JoinHandle,
};
use tracing::info;
use tracing_subscriber::{
    filter::Directive, fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
    EnvFilter,
};
use webhook::WebhookServer;

pub struct AllInOneInstance {
    pub stop: Sender<Sender<()>>,
    pub runtime: Runtime,
}

fn load_settings_for<T: Default + DeserializeOwned + Clone>(
    settings: &str,
    name: &str,
) -> Result<Settings<T>> {
    let value: Value = serde_json::from_str(settings)?;
    let section: T = serde_json::from_value(value.get(name).unwrap().to_owned())?;
    let mut settings: Settings<T> = serde_json::from_value(value)?;
    settings.config = section;

    Ok(settings)
}

// Start a component
async fn start_component<T: Component>(
    settings: String,
    aio: &mut Vec<Sender<()>>,
) -> JoinHandle<()> {
    let name = T::SERVICE_NAME;
    let instance = T::new();

    let (stop, signal) = oneshot::channel();

    aio.push(stop);

    tokio::spawn(async move {
        let config = load_settings_for::<<T as Component>::Config>(&settings, name).unwrap();
        instance.start(config, signal).await.unwrap();
    })
}

#[no_mangle]
/// Loads the config json using the nova shared config loader
pub extern "C" fn load_config() -> *const libc::c_char {
    let mut builder = Config::builder();

    builder = builder.add_source(File::with_name("config/default"));
    let mode = env::var("ENV").unwrap_or_else(|_| "development".into());
    info!("Configuration Environment: {}", mode);

    builder = builder.add_source(File::with_name(&format!("config/{}", mode)).required(false));
    builder = builder.add_source(File::with_name("config/local").required(false));

    let env = Environment::with_prefix("NOVA").separator("__");
    // we can configure each component using environment variables
    builder = builder.add_source(env);

    let config: Value = builder.build().unwrap().try_deserialize().unwrap();
    let s = serde_json::to_string(&config).unwrap();

    let c_str_song = CString::new(s).unwrap();
    c_str_song.into_raw()
}

#[no_mangle]
/// Initialise les logs des composants de nova
/// Utilise la crate `pretty_log_env`
pub extern "C" fn init_logs() {}

#[no_mangle]
/// Stops a nova instance
pub unsafe extern "C" fn stop_instance(instance: *mut AllInOneInstance) {
    let instance = Box::from_raw(instance);
    let (tell_ready, ready) = oneshot::channel();
    instance.stop.send(tell_ready).unwrap();
    ready.blocking_recv().unwrap();
    instance.runtime.shutdown_timeout(Duration::from_secs(5));
}

#[no_mangle]
/// Initialized a new nova instance and an async runtime (tokio reactor)
/// Dont forget to stop this instance using `stop_instance`
pub unsafe extern "C" fn start_instance(config: *const libc::c_char) -> *mut AllInOneInstance {
    let buf_name = unsafe { CStr::from_ptr(config).to_bytes() };
    let settings = String::from_utf8(buf_name.to_vec()).unwrap();
    let (stop, trigger_stop) = oneshot::channel();

    // Initialize a tokio runtime
    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
        global::set_text_map_propagator(TraceContextPropagator::new());
        let tracer =
            opentelemetry_otlp::new_pipeline()
                .tracing()
                .with_trace_config(trace::config().with_resource(Resource::new(vec![
                    KeyValue::new("service.name", "all-in-one"),
                ])))
                .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_env())
                .install_batch(opentelemetry::runtime::Tokio)
                .unwrap();

        let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(telemetry)
            .with(
                EnvFilter::builder()
                    .with_default_directive(Directive::from_str("info").unwrap())
                    .from_env()
                    .unwrap(),
            )
            .init();
        // Start the gateway server

        let mut aio = vec![];
        let mut handles = vec![];

        // Start components
        handles.push(start_component::<GatewayServer>(settings.clone(), &mut aio).await);
        handles
            .push(start_component::<RatelimiterServerComponent>(settings.clone(), &mut aio).await);
        handles.push(start_component::<ReverseProxyServer>(settings.clone(), &mut aio).await);
        handles.push(start_component::<WebhookServer>(settings.clone(), &mut aio).await);

        // wait for exit
        let done: Sender<()> = trigger_stop.await.unwrap();

        // Tell all the threads to stop.
        while let Some(stop_signal) = aio.pop() {
            stop_signal.send(()).unwrap();
        }

        // Wait for all the threads to finish.
        while let Some(handle) = handles.pop() {
            handle.await.unwrap();
        }

        done.send(()).unwrap();
    });
    Box::into_raw(Box::new(AllInOneInstance { stop, runtime: rt }))
}
