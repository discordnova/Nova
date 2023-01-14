#![allow(clippy::missing_safety_doc)]
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
    .or(Some(ptr::null::<c_char>() as *mut c_char))
    .expect("something has gone terribly wrong")
}

#[no_mangle]
pub unsafe extern "C" fn stop_instance(instance: *mut AllInOneInstance) {
    wrap_result(move || {
        let mut instance = Box::from_raw(instance);
        let handles = take(&mut instance.handles);
        instance.runtime.block_on(async move {
            for (name, sender, join) in handles {
                debug!("Halting component {}", name);
                let _ = sender
                    .send(())
                    .map_err(|_| error!("Component {} is not online", name));
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

/// # Panics
/// Panics if an incorrect `RUST_LOG` variables is specified.
#[no_mangle]
pub unsafe extern "C" fn create_instance(config: *mut c_char) -> *mut AllInOneInstance {
    // Returning a null pointer (unaligned) is expected.
    #[allow(clippy::cast_ptr_alignment)]
    wrap_result(move || {
        let value = CString::from_raw(config);
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
                    .with_default_directive(Directive::from_str("info").expect(""))
                    .from_env()
                    .unwrap(),
            )
            .init();

        // Error handling task
        runtime.spawn(async move {
            while let Some(error) = errors.recv().await {
                handle_error(&error);
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
            error_sender,
            &runtime,
        )?);

        let all_in_one = Box::into_raw(Box::new(AllInOneInstance { runtime, handles }));

        Ok(all_in_one)
    })
    .or(Some(ptr::null::<AllInOneInstance>() as *mut AllInOneInstance))
    .expect("something has gone terribly wrong")
}
