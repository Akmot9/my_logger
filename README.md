# My Logger Crate

[![Crates.io](https://img.shields.io/crates/v/my_logger.svg)](https://crates.io/crates/my_logger)

My Logger is a custom logging crate for Rust that provides a simple `log!` macro to log messages to a file with timestamps.

## Installation

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
my_logger = "0.1.0"  # Replace with the latest version available
```

## Usage
First, import the log! macro from the crate:
```rust
use my_logger::log;
```
Then, you can use the log! macro to log messages:
```rust
fn main() {
    log!("This is a log message.");
    log!("Another log message with a value: {}", 42);
}
```
The log messages will be written to a file named "file.log" in the current directory, and each log entry will include a timestamp.

## Exemple
The log file (file.log) will contain a line like this:
```log
[2023-08-04T12:34:56Z] This is a log message.
[2023-08-04T12:34:56Z] Another log message with a value: 42
``````
