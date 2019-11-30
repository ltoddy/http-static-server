use std::env::current_dir;
use std::fmt;
use std::path::PathBuf;

use async_std::fs;
use async_std::io;

use crate::method::HttpMethod;
use crate::request::HttpRequest;
use crate::response::content::ContentType;
use crate::response::headers::Headers;
use crate::status::HttpStatusCode;
use crate::Result;

pub mod content;
pub mod headers;

#[derive(Debug)]
pub struct HttpResponse {
    status: HttpStatusCode,
    headers: Headers,
    body: Option<Vec<u8>>,
}

impl HttpResponse {
    pub fn new() -> Self {
        Self {
            status: HttpStatusCode::Ok,
            headers: Headers::new(),
            body: None,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let HttpResponse { status, headers, body } = self;
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

impl fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "HTTP/1.1 {}", self.status)
    }
}

pub async fn process_request(request: HttpRequest) -> Result<HttpResponse> {
    match request.method {
        HttpMethod::Get => handle_get_request(request).await,
        _ => unimplemented!(),
    }
}

async fn handle_get_request(request: HttpRequest) -> Result<HttpResponse> {
    let current_dir = current_dir().unwrap();
    let root = current_dir.to_str().unwrap(); // TODO
    let path = format!("{}{}", root, request.uri.to_str().unwrap());
    let path = PathBuf::from(path);

    let mut response = HttpResponse::new();
    let contents = fs::read(&path).await;
    match contents {
        Ok(contents) => {
            response.body = Some(contents);
            let file_ext = path.extension().unwrap();
            response.headers.content_type = Some(ContentType::from_file_extension(file_ext)?);

            Ok(response)
        }
        Err(e) => {
            response.status = match e.kind() {
                io::ErrorKind::NotFound => {
                    response.body = Some(vec![]);
                    HttpStatusCode::NotFound
                }
                io::ErrorKind::PermissionDenied => HttpStatusCode::Forbidden,
                _ => HttpStatusCode::InternalServerError,
            };
            Ok(response)
        }
    }
}
