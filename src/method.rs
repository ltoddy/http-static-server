use std::error;
use std::fmt;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
#[allow(unused_must_use)]
pub enum HttpMethod {
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
    pub fn from_str(m: String) -> Result<Self, InvalidHttpMethod> {
        match m.as_str() {
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

    pub fn as_str(&self) -> &str {
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

#[derive(Debug)]
pub struct InvalidHttpMethod(String);

impl<'a> fmt::Display for InvalidHttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid http method: {}", self.0)
    }
}

impl<'a> error::Error for InvalidHttpMethod {}
