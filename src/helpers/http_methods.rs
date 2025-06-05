use std::fmt::{self, Display, Formatter};

// enum for the HTTP methods available
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum HttpMethod {
    DELETE,
    GET,
    HEAD,
    OPTIONS,
    PATCH,
    POST,
    PUT,
}

// string representation of HttpMethod
impl HttpMethod {
    pub fn from_string(method: String) -> HttpMethod {
        match method.to_uppercase().as_str() {
            "DELETE" => HttpMethod::DELETE,
            "GET" => HttpMethod::GET,
            "HEAD" => HttpMethod::HEAD,
            "OPTIONS" => HttpMethod::OPTIONS,
            "PATCH" => HttpMethod::PATCH,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            _ => HttpMethod::POST,
        }
    }
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            HttpMethod::DELETE => "DELETE",
            HttpMethod::GET => "GET",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
        };
        write!(f, "{}", s)
    }
}
