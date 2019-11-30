use std::error;
use std::fmt;
use std::path::PathBuf;
use std::str;

use crate::method::HttpMethod;

#[derive(Debug)]
struct RequestLineNotFound;

impl fmt::Display for RequestLineNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "request line not found.")
    }
}

impl error::Error for RequestLineNotFound {}

#[derive(Debug)]
struct InvalidRequest<'a>(&'a str);

impl<'a> fmt::Display for InvalidRequest<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid request: {}", self.0)
    }
}

impl<'a> error::Error for InvalidRequest<'a> {}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub uri: PathBuf,
    pub version: String,
}

impl fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {} {}\r", self.method.as_str(), self.uri.display(), self.version)
    }
}

impl HttpRequest {
    pub fn new(method: HttpMethod, uri: PathBuf, version: String) -> Self {
        Self { method, uri, version }
    }

    fn with_method(&mut self, method: HttpMethod) -> &mut Self {
        self.method = method;
        self
    }

    fn with_uri(&mut self, uri: PathBuf) -> &mut Self {
        self.uri = uri;
        self
    }

    fn with_version(&mut self, version: String) -> &mut Self {
        self.version = version;
        self
    }
}

impl Default for HttpRequest {
    fn default() -> Self {
        Self {
            method: HttpMethod::Get,
            uri: PathBuf::new(),
            version: String::new(),
        }
    }
}

pub fn parse_request(buffer: Vec<u8>) -> Result<HttpRequest, Box<dyn error::Error>> {
    let request = String::from_utf8(buffer)?;
    let request_line = request.lines().next().ok_or("request line not found")?;

    let mut parts = request_line.split_whitespace();

    let method = HttpMethod::from_str(parts.next().ok_or("http method not specified")?.to_owned())?;
    let uri = PathBuf::from(parts.next().ok_or("uri not specified")?);
    let version = parts.next().ok_or("http version not specified")?.to_owned();

    let mut request = HttpRequest::default();

    request.with_method(method).with_uri(uri).with_version(version);

    Ok(request)
}
