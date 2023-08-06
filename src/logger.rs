/// This is a custom logging library for Rust.
/// Logs a message to a file with a timestamp.
///
/// # Example
///
/// ```rust
/// use my_logger::log;
///
/// fn main() {
///     log!("This is a log message.");
///     log!("Another log message with a value: {}", 42);
/// }
/// ```
///
/// The log messages will be written to a file named "file.log" in the current directory
//use chrono::Utc;
// Define your custom logging macro here
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        use chrono::Utc;
        use std::fs::OpenOptions;
        use std::io::Write;
        let log_message = format!($($arg)*);
        let now = Utc::now();
        let log_line = format!("[{}] {}\n", now, log_message);

        if let Ok(mut file) = OpenOptions::new()
            .append(true)
            .create(true)
            .open("file.log")
        {
            if let Err(e) = writeln!(file, "{}", log_line) {
                eprintln!("Error writing to log file: {}", e);
            }
        } else {
            eprintln!("Error opening log file!");
        }
    }};
}
