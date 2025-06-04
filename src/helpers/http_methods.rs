// enum for the HTTP methods available
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
	pub fn to_string(&self) -> String {
		return match self {
			HttpMethod::DELETE => "DELETE".to_string(),
			HttpMethod::GET => "GET".to_string(),
			HttpMethod::HEAD => "HEAD".to_string(),
			HttpMethod::OPTIONS => "OPTIONS".to_string(),
			HttpMethod::PATCH => "PATCH".to_string(),
			HttpMethod::POST => "POST".to_string(),
			HttpMethod::PUT => "PUT".to_string(),
		};
	}
}