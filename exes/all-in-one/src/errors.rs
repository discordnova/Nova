use std::cell::RefCell;

use anyhow::Result;
use tracing::error;

thread_local! {
    pub static ERROR_HANDLER: std::cell::RefCell<Option<unsafe extern "C" fn(libc::c_int, *mut libc::c_char)>>  = RefCell::new(None);
}

/// Update the most recent error, clearing whatever may have been there before.
#[must_use] pub fn stacktrace(err: &anyhow::Error) -> String {
    format!("{err}")
}

pub fn wrap_result<T, F>(func: F) -> Option<T>
where
    F: Fn() -> Result<T>,
{
    let result = func();

    match result {
        Ok(ok) => Some(ok),
        Err(error) => {
            // Call the handler
            handle_error(&error);
            None
        }
    }
}

/// # Panics
/// Panics if the stacktrace size is > than an i32
pub fn handle_error(error: &anyhow::Error) {
    ERROR_HANDLER.with(|val| {
        let mut stacktrace = stacktrace(error);

        error!("Error emitted: {}", stacktrace);
        if let Some(func) = *val.borrow() {
            // Call the error handler
            unsafe {
                func(
                    (stacktrace.len() + 1).try_into().unwrap(),
                    stacktrace.as_mut_ptr().cast::<i8>(),
                );
            }
        }
    });
}

#[cfg(test)]
mod tests {
    // todo
}
