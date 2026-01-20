//! A simple logger [`Builder`] that writes all logs to a static buffer.
use std::io::Write;
use std::sync::{Arc, Mutex, OnceLock};
use env_logger::Builder;

/// Static buffer to store all logs to as text.
static BUFFER: OnceLock<Arc<Mutex<String>>> = OnceLock::new();

/// Returns a reference to the [`BUFFER`].
fn get_buf() -> &'static Arc<Mutex<String>> {
    BUFFER.get().unwrap()
}

/// Returns a cloned [`String`] of the [`BUFFER`].
pub fn get_logs() -> String {
    get_buf().lock().unwrap().clone()
}

/// Returns a logger [`Builder`] that automatically writes logs to the [`BUFFER`].
pub fn get_builder() -> Builder {
    let _ = BUFFER.set(Arc::new(Mutex::new(String::new())));

    let mut builder = Builder::from_default_env();
    builder
        .format(move |buf, record| {
            let line = format!("[{}] {}\n", record.level(), record.args());

            {
                let mut guard = get_buf().lock().unwrap();
                guard.push_str(&line);
            }

            buf.write_all(line.as_bytes())
        })
        .write_style(env_logger::WriteStyle::Never);

    builder
}
