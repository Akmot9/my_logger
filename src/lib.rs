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

