use crate::HttpMethod;
use std::collections::HashMap;

pub struct Request {
	pub raw: String,
	pub method: HttpMethod,
	pub path: String,
	pub split_path: Vec<String>,
	pub headers: HashMap<String, String>,
	pub body: String,
}


impl Request {
	pub fn new(raw: String, method: HttpMethod, path: String, split_path: Vec<String>, headers: HashMap<String, String>, body: String) -> Request {
		Self {
			raw: raw,
			method: method,
			path: path,
			split_path: split_path,
			headers: headers,
			body: body
		}
	}
}


// fn get_method(raw: String) -> HttpMethod {

// }