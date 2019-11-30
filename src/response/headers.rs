use crate::response::content::ContentType;

#[derive(Debug)]
pub struct Headers {
    pub content_type: Option<ContentType>,
}

impl Headers {
    pub fn new() -> Self {
        Self { content_type: None }
    }
}
