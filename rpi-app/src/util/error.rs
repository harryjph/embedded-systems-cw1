use anyhow::Error;

/// The i2cdev error types produce lifetime errors when you try to return them.
/// This is a flaw of the crate's structure. This is a hack to work around it.
pub fn stringify_error<E: std::error::Error>(error: E) -> Error {
    Error::msg(format!("{error}"))
}

pub trait DescribeError<T> {
    fn describe_error(self, description: &'static str) -> Result<T, Error>;
}

impl<T, E: Into<Error>> DescribeError<T> for Result<T, E> {
    fn describe_error(self, description: &'static str) -> Result<T, Error> {
        self.map_err(move |e| e.into().context(description))
    }
}
