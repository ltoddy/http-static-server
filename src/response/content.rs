use std::{error, fmt};

/// see more: https://developer.mozilla.org/zh-CN/docs/Web/HTTP/Basics_of_HTTP/MIME_types
pub enum ContentType {
    HTML,
    CSS,
    TEXT,
    PNG,
    JPEG,
    SVG,
    PDF,
    GIF,
}

impl ContentType {
    pub fn from(source: String) -> Result<Self, InvalidContentType> {
        match source.as_str() {
            "html" => Ok(ContentType::HTML),
            "css" => Ok(ContentType::CSS),
            "txt" => Ok(ContentType::TEXT),
            "png" => Ok(ContentType::PNG),
            "jpeg" => Ok(ContentType::PNG),
            "svg" => Ok(ContentType::SVG),
            "pdf" => Ok(ContentType::PDF),
            "gif" => Ok(ContentType::GIF),

            other => Err(InvalidContentType(source)),
        }
    }
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            ContentType::HTML => String::from("text/html"),
            ContentType::CSS => String::from("text/css"),
            ContentType::TEXT => String::from("text/plain"),
            ContentType::PNG => String::from("image/png"),
            ContentType::JPEG => String::from("image/jpeg"),
            ContentType::SVG => String::from("image/svg+xml"),
            ContentType::PDF => String::from("application/pdf"),
            ContentType::GIF => String::from("image/gif"),
        }
    }
}

#[derive(Debug)]
pub struct InvalidContentType(String);

impl error::Error for InvalidContentType {}

impl fmt::Display for InvalidContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid content type: {}", self.0)
    }
}
