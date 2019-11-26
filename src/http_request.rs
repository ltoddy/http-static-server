use std::fmt;
use std::path::{Path, PathBuf};
use std::str;

use crate::error::HttpStaticServerError;
use crate::error::Result;
use crate::http_method::HttpMethod;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub uri: PathBuf,
    pub version: String,
}

impl fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{} {} {}\r",
            self.method.as_str(),
            self.uri.display(),
            self.version
        )
    }
}

impl HttpRequest {
    pub fn new<Uri: AsRef<Path>>(method: HttpMethod, uri: Uri, version: String) -> Self {
        Self {
            method,
            uri: uri.as_ref().to_path_buf(),
            version,
        }
    }
}

fn get_request_line(buffer: &[u8]) -> Result<&str> {
    let request = str::from_utf8(buffer)?;
    Ok(request.lines().next().ok_or(HttpStaticServerError::RequestLineNotFound)?)
}

pub fn parse_request(request: &str) -> Result<HttpRequest> {
    // TODO
    unimplemented!()
}
