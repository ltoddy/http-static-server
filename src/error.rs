use async_std::io;
use failure::Fail;

/// Error type for `http-static-server`
#[derive(Debug, Fail)]
pub enum HttpStaticServerError {
    /// IO error
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
}

impl From<io::Error> for HttpStaticServerError {
    fn from(err: io::Error) -> Self {
        HttpStaticServerError::Io(err)
    }
}

/// Result type for `http-static-server`
pub type Result<T> = std::result::Result<T, HttpStaticServerError>;
