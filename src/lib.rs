//! This is a custom logging library for Rust.
//!
//! It provides a `log!` macro to log messages to a file with timestamps.
//!
//! Example:
//! ```
//! use my_logger::log;
//!
//! fn main() {
//!     log!("This is a log message.");
//!     log!("Another log message with a value: {}", 42);
//! }
//! ```
//!
//! The log messages will be written to a file named "file.log" in the current directory.
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

// Export the logger macro from the `logger` module
pub mod logger;
pub use logger::*;

// Add these imports at the beginning of the test module
#[cfg(test)]
use std::fs;
#[cfg(test)]
use std::io::BufRead;
#[cfg(test)]
use std::io::BufReader;

#[test]
fn test_logging_macros() {
    // Clear the log file before running the test
    let _ = fs::remove_file("file.log");

    // Log some messages using the log! macro
    log!("Test message 1");
    log!("Test message 2");

    // Read the log file and count the number of lines
    let file = fs::File::open("file.log").expect("Failed to open log file");
    let reader = BufReader::new(file);
    let line_count = reader.lines().count();

    // Divide line_count by 2 to get the actual number of logged messages
    let actual_message_count = line_count / 2;

    // Assert that the actual number of messages matches our expectations
    assert_eq!(actual_message_count, 2);

}