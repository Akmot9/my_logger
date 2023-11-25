# My Logger Crate

[![Crates.io](https://img.shields.io/crates/v/my_logger.svg)](https://crates.io/crates/my_logger)

My Logger is a custom logging crate for Rust that provides a simple `log!` macro to log messages to a file with timestamps. it doesn't use unsafe rust.

## Installation

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
my_logger = "0.2.1"  # Replace with the latest version available
```

## Usage
First, import the log! macro from the crate:
```rust
use my_logger::{log, logw, logd};
```
Then, you can use the log! macro to log messages:
```rust
fn main() {
    log!("This is a log message.");
    log!("Another log message with a value: {}", 42);
    let err = "Something went wrong!";
    log!("error: {}", err);
    logd!("debug: {}", err);
    logw!("warning: {}", err);
}
```
The log messages will be written to a file named "file.log" in the current directory, and each log entry will include a timestamp.

## Exemple
The log file (file.log) will contain a line like this:
```log
[2023-08-18 21:48:01] This is a log message.
[2023-08-18 21:48:02] Another log message with a value: 42
[2023-08-18 21:48:03] error: Something went wrong!
[2023-08-18 21:48:04] warning: Something went wrong!
``````
