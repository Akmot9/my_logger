/// This is a custom logging library for Rust.
///
/// Logs messages to a file with timestamps and provides different macros for logging.
///
/// # Example
///
/// ```rust
/// use my_logger::{log, logd, logw};
///
/// fn main() {
///     log!("This is a log message.");
///     logd!("This is a debug log message (displayed in CLI only).");
///     logw!("This is a warning log message (logged to file only).");
/// }
/// ```
///
/// The log messages will be written to a file named "file.log" in the current directory.
///
/// # Macros
///
/// - `log!` - Logs a message to both CLI and file with a timestamp.
/// - `logd!` - Logs a message to CLI only without logging to the file.
/// - `logw!` - Logs a message to the file only without displaying in CLI.
///
/// # Notes
///
/// - The `log!` and `logd!` macros use the same timestamp formatting.
/// - The `logw!` macro logs directly to the file without printing in the CLI.
///
/// # Safety
///
/// The macros involve file I/O, which can potentially lead to data loss or corruption
/// if not handled correctly. Ensure that the underlying file operations are safe and
/// error handling is appropriately implemented.

// Export the logger macro from the `logger` module
pub mod logger;
pub use logger::*;




