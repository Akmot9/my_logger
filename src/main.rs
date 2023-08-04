// Import the `log` macro from the library
use my_logger::log;

fn main() {
    // Use the log macro to log an error message
    let err = "Something went wrong!";
    log!("error: {}", err);
}
