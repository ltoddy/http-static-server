use std::string;
use std::str;

use async_std::io;
use failure::Fail;

/// Error type for `http-static-server`
#[derive(Debug, Fail)]
pub enum HttpStaticServerError {
    #[fail(display = "{}", _0)]
    Io(#[fail(cause)] io::Error),

    #[fail(display = "Invalid HTTP method: {}", _0)]
    HttpMethod(String),

    #[fail(display = "Invalid Uri: {}", _0)]
    Uri(String),

    #[fail(display = "Request-Line not found")]
    RequestLineNotFound,

    #[fail(display = "UTF-8 error: {}", _0)]
    Utf8(#[fail(cause)] str::Utf8Error),
}

impl From<io::Error> for HttpStaticServerError {
    fn from(err: io::Error) -> Self {
        HttpStaticServerError::Io(err)
    }
}

impl From<str::Utf8Error> for HttpStaticServerError {
    fn from(err: str::Utf8Error) -> Self {
        HttpStaticServerError::Utf8(err)
    }
}

/// Result type for `http-static-server`
pub type Result<T> = std::result::Result<T, HttpStaticServerError>;
