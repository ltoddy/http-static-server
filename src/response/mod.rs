use crate::response::headers::Headers;
use crate::status::HttpStatusCode;

pub mod content;
pub mod headers;

#[derive(Debug)]
pub struct Response {
    status: HttpStatusCode,
    headers: Headers,
    body: Option<Vec<u8>>,
}

impl Response {
    pub fn new() -> Self {
        Self {
            status: HttpStatusCode::Ok,
            headers: Headers::new(),
            body: None,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let Response { status, headers, body } = self;
        let Headers { content_type } = headers;

        let mut result = format!("HTTP/1.1 {}\r\n", status);
        if let Some(content_type) = content_type {
            result.push_str(&format!("Content-Type: {}\r\n\r\n", content_type.to_string()));
        }

        let mut bytes = result.as_bytes().to_vec();

        if let Some(body) = body {
            bytes.append(&mut body.to_owned());
        }

        bytes
    }
}
