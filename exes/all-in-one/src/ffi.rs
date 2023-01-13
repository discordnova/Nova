use std::{
    ffi::{c_char, c_int, CString},
    mem::take,
    ptr,
    str::FromStr,
    time::Duration,
};

use gateway::GatewayServer;
use opentelemetry::{global::set_text_map_propagator, sdk::propagation::TraceContextPropagator};
use ratelimit::RatelimiterServerComponent;
use rest::ReverseProxyServer;
use tokio::{runtime::Runtime, sync::mpsc};
use tracing::{debug, error};
use tracing_subscriber::{
    filter::Directive, fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
    EnvFilter,
};
use webhook::WebhookServer;

use crate::{
    errors::{handle_error, wrap_result, ERROR_HANDLER},
    utils::{load_config_file, start_component, AllInOneInstance},
};

#[no_mangle]
pub unsafe extern "C" fn set_error_handler(func: unsafe extern "C" fn(c_int, *mut c_char)) {
    debug!("Setting error handler");
    ERROR_HANDLER.with(|prev| {
        *prev.borrow_mut() = Some(func);
    });
}

#[no_mangle]
/// Loads the config json using the nova shared config loader
pub extern "C" fn load_config() -> *mut c_char {
    wrap_result(move || {
        let config = serde_json::to_string(&load_config_file()?)?;
        let c_str_song = CString::new(config)?;
        Ok(c_str_song.into_raw())
    })
    .or(Some(ptr::null::<i8>() as *mut i8))
    .expect("something has gone terribly wrong")
}

#[no_mangle]
pub extern "C" fn stop_instance(instance: *mut AllInOneInstance) {
    wrap_result(move || {
        let mut instance = unsafe { Box::from_raw(instance) };
        let handles = take(&mut instance.handles);
        instance.runtime.block_on(async move {
            for (name, sender, join) in handles {
                debug!("Halting component {}", name);
                let _ = sender
                    .send(())
                    .or_else(|_| Err(error!("Component {} is not online", name)));
                match join.await {
                    Ok(_) => {}
                    Err(error) => error!("Task for component {} panic'ed {}", name, error),
                };
                debug!("Component {} halted", name);
            }
        });

        instance.runtime.shutdown_timeout(Duration::from_secs(5));

        Ok(())
    });
}

#[no_mangle]
pub extern "C" fn create_instance(config: *mut c_char) -> *mut AllInOneInstance {
    wrap_result(move || {
        let value = unsafe { CString::from_raw(config) };
        let json = value.to_str()?;

        // Main stop signal for this instance
        let (error_sender, mut errors) = mpsc::channel(50);
        let mut handles = vec![];

        let runtime = Runtime::new()?;

        // Setup the tracing system
        set_text_map_propagator(TraceContextPropagator::new());
        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(
                EnvFilter::builder()
                    .with_default_directive(Directive::from_str("info").unwrap())
                    .from_env()
                    .unwrap(),
            )
            .init();

        // Error handling task
        runtime.spawn(async move {
            while let Some(error) = errors.recv().await {
                handle_error(error)
            }
        });

        handles.push(start_component::<GatewayServer>(
            json,
            error_sender.clone(),
            &runtime,
        )?);

        std::thread::sleep(Duration::from_secs(1));

        handles.push(start_component::<RatelimiterServerComponent>(
            json,
            error_sender.clone(),
            &runtime,
        )?);

        std::thread::sleep(Duration::from_secs(1));

        handles.push(start_component::<ReverseProxyServer>(
            json,
            error_sender.clone(),
            &runtime,
        )?);

        std::thread::sleep(Duration::from_secs(1));

        handles.push(start_component::<WebhookServer>(
            json,
            error_sender.clone(),
            &runtime,
        )?);

        let all_in_one = Box::into_raw(Box::new(AllInOneInstance { runtime, handles }));

        Ok(all_in_one)
    })
    .or(Some(ptr::null::<i8>() as *mut AllInOneInstance))
    .expect("something has gone terribly wrong")
}
