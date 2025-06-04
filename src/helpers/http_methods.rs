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