use std::fmt;

use crate::error::HttpStaticServerError::InvalidHttpMethod;
use crate::error::Result;

#[derive(Debug, PartialOrd, PartialEq)]
pub(crate) enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Options,
    Head,
    Connect,
    Trace,
}

impl HttpMethod {
    fn from(m: &str) -> Result<Self> {
        match m {
            "GET" => Ok(HttpMethod::Get),
            "Post" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "DELETE" => Ok(HttpMethod::Delete),
            "OPTIONS" => Ok(HttpMethod::Options),
            "HEAD" => Ok(HttpMethod::Head),
            "CONNECT" => Ok(HttpMethod::Connect),
            "TRACE" => Ok(HttpMethod::Trace),
            other => Err(InvalidHttpMethod(other.to_owned())),
        }
    }

    fn as_str(&self) -> &str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Options => "OPTIONS",
            HttpMethod::Head => "HEAD",
            HttpMethod::Connect => "CONNECT",
            HttpMethod::Trace => "TRACE",
        }
    }
}
