use std::error::Error;

mod stream;

pub use stream::*;

/// The i2cdev error types produce lifetime errors when you try to return them.
/// This is a flaw of the crate's structure. This is a hack to work around it.
pub fn stringify_error<E: Error>(error: E) -> Box<dyn Error> {
    format!("{error}").into()
}
