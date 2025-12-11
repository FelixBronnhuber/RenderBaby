use std::io::Write;
use std::sync::{Arc, Mutex, OnceLock};
use env_logger::Builder;

static BUFFER: OnceLock<Arc<Mutex<String>>> = OnceLock::new();

fn get_buf() -> &'static Arc<Mutex<String>> {
    BUFFER.get().unwrap()
}

pub fn get_logs() -> String {
    get_buf().lock().unwrap().clone()
}

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
