use std::fmt;

/// see more: https://developer.mozilla.org/zh-CN/docs/Web/HTTP/Status
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum HttpStatusCode {
    Continue,

    Ok,
    Created,
    Accepted,

    MovedPermanently,
    NotModified,
    TemporaryRedirect,
    PermanentRedirect,

    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllow,

    InternalServerError,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
}

impl fmt::Display for HttpStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpStatusCode::Continue => write!(f, "100 Continue"),

            HttpStatusCode::Ok => write!(f, "200 OK"),
            HttpStatusCode::Created => write!(f, "201 Created"),
            HttpStatusCode::Accepted => write!(f, "202 Accepted"),

            HttpStatusCode::MovedPermanently => write!(f, "301 Moved Permanently"),
            HttpStatusCode::NotModified => write!(f, "304 Not Modified"),
            HttpStatusCode::TemporaryRedirect => write!(f, "307 Temporary Redirect"),
            HttpStatusCode::PermanentRedirect => write!(f, "308 Permanent Redirect"),

            HttpStatusCode::BadRequest => write!(f, "400 Bad Request"),
            HttpStatusCode::Unauthorized => write!(f, "401 Unauthorized"),
            HttpStatusCode::Forbidden => write!(f, "403 Forbidden"),
            HttpStatusCode::NotFound => write!(f, "404 Not Found"),
            HttpStatusCode::MethodNotAllow => write!(f, "405 Method Not Allowed"),

            HttpStatusCode::InternalServerError => write!(f, "500 Internal Server Error"),
            HttpStatusCode::BadGateway => write!(f, "502 Bad Gateway"),
            HttpStatusCode::ServiceUnavailable => write!(f, "503 Service Unavailable"),
            HttpStatusCode::GatewayTimeout => write!(f, "504 Gateway Timeout"),
        }
    }
}
