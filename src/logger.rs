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
        use chrono::Local;
        use std::fs::OpenOptions;
        use std::io::Write;
        let log_message = format!($($arg)*);
        let now = Local::now();
        let formatted = format!("{}", now.format("%Y-%m-%d %H:%M:%S")) ;
        let log_line = format!("[{}] {}\n", formatted, log_message);
        println!("{log_line}");

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


/// Logs a debug message to the terminal with a timestamp.
///
/// Debug messages logged with this macro will be displayed in the CLI only.
/// They will not be written to the log file.
///
/// # Example
///
/// ```rust
/// use my_logger::logd;
///
/// fn main() {
///     logd!("This is a debug log message.");
/// }
/// ```
#[macro_export]
macro_rules! logd {
    ($($arg:tt)*) => {{
        use chrono::Local;
        let log_message = format!($($arg)*);
        let now = Local::now();
        let formatted = format!("{}", now.format("%Y-%m-%d %H:%M:%S")) ;
        let log_line = format!("[{}] {}\n", formatted, log_message);
        println!("{log_line}");

    }};
}


/// Logs a warning message to the log file with a timestamp.
///
/// Warning messages logged with this macro will be written to the log file only.
/// They will not be displayed in the CLI.
///
/// # Example
///
/// ```rust
/// use my_logger::logw;
///
/// fn main() {
///     logw!("This is a warning log message.");
/// }
/// ```
#[macro_export]
macro_rules! logw {
    ($($arg:tt)*) => {{
        use chrono::Local;
        use std::fs::OpenOptions;
        use std::io::Write;
        let log_message = format!($($arg)*);
        let now = Local::now();
        let formatted = format!("{}", now.format("%Y-%m-%d %H:%M:%S")) ;
        let log_line = format!("[{}] {}\n", formatted, log_message);

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