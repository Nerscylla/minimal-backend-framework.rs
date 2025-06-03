// imports
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::collections::HashMap;
use std::process::exit;



// enum for the HTTP methods available
enum HttpMethod {
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
	fn to_string(&self) -> String {
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



// Represents a web application with configurable listener settings and route handlers.
struct App {
	listener_port: u16,
	listener_base_address: String,
	routes: HashMap<String, HashMap<String, Box<dyn Fn() -> String>>>,
	buffer_size: u64,
	_delete: HttpMethod,
    _get: HttpMethod,
    _head: HttpMethod,
    _options: HttpMethod,
    _patch: HttpMethod,
    _post: HttpMethod,
    _put: HttpMethod,
}


// actual functional implementation
impl App {
	// constructor function
	fn new(listener_port: u16, listener_base_address: String) -> Self {
		Self {
			listener_port,
			listener_base_address,
			buffer_size: 262144,
			routes: HashMap::new(),
			_delete: HttpMethod::DELETE,
			_get: HttpMethod::GET,
			_head: HttpMethod::HEAD,
			_options: HttpMethod::OPTIONS,
			_patch: HttpMethod::PATCH,
			_post: HttpMethod::POST,
			_put: HttpMethod::PUT,
		}
	}


	// function to register handlers for a specific route using a specific method
	fn reg_path(&mut self, method: HttpMethod, path: String, handler_function: Box<dyn Fn() -> String>) {
		// get the uppercase method as a string
		let method_upper: String = method.to_string();
		// add the route in self/routes/<method>/<path>:handler_function
		self.routes
			.entry(method_upper.clone())
			.or_insert_with(HashMap::new)
			.insert(path.clone(), handler_function);
		// log the registration of the route
		println!("Registered route: {} {}", method_upper, path)
	}


	// listening function
	fn listen(&mut self) {
		// create a listener with helper function
		let listener: TcpListener = setup_listener(self);

		// loop through incoming streams
		for stream in listener.incoming() {
			// error handling the stream
			match stream {
				Ok(stream) => {
					// handle Ok streams
					handle_incoming_stream(self, stream);
				}
				Err(e) => {
					// error logging
					eprintln!("Failed to accept connection: {}", e);
				},
			}
		}
	}
}


// hanlder function for streams coming in
fn handle_incoming_stream(app: &mut App, mut stream: TcpStream) {
	// create a buffer to read into
	let mut buffer = vec![0u8; app.buffer_size as usize];
	// try reading into buffer
	let bytes_read = match stream.read(&mut buffer) {
		Ok(n) => n,
		Err(e) => {
			// error logging and return if read error occurs
			eprintln!("Failed to read from stream: {}", e);
			return;
		}
	};

	// move buffer into string
	let request = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

	// seperate the request into lines
	let request_lines: Vec<&str> = request.lines().collect();

	// Separate the first element (request line) from the rest
	let (first_line, _request_lines) = match request_lines.split_first() {
		Some((first, rest)) => (first, rest),
		None => {
			eprintln!("Empty request received");
			return;
		}
	};

	// split the first line up into words
	let split_first_line: Vec<String> = first_line
		.split(' ')
		.map(|s| s.to_string())
		.collect();
	// extract method and path from request
	let method = &split_first_line[0];
	let mut path = split_first_line[1].as_str();
	// Remove trailing slash if present (but not for root "/")
	while path.len() > 1 && path.ends_with('/') {
		path = &path[..path.len() - 1];
	}


	// start building the response
	let response_text: String;
	// response function
	let response_function = app
		.routes
		.get(method)
		.and_then(|method_map| method_map.get(path));
	if let Some(handler) = response_function {
		// execute handler
		response_text = handler();
	} else {
		// no handler
		response_text = "404 not found".to_string();
	}
	// put response into HTTP response
	let response = format!(
		"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: {}\r\n\r\n{}",
		response_text.len(),
		response_text
	);
	// send back HTTP over tcp stream
	match stream.write_all(response.as_bytes()) {
		Ok(_) => {}
		Err(e) => {
			eprintln!("Failed to write response to stream: {}", e);
			return
		}
	}
}


// helper function to set up the listener to prevent unreadable nesting
fn setup_listener(app: &mut App) -> TcpListener {
	// try to bind the new listener
	return match TcpListener::bind(format!("{0}:{1}", app.listener_base_address, app.listener_port)) {
		Ok(listener) => {
			// log successful listening and return listener
			println!("Successfully listening at {}:{}", app.listener_base_address, app.listener_port);
			listener
		}
		Err(e) => {
			// log error binding and return code 99
			eprintln!("Failed to bind to {}:{} - {}", app.listener_base_address, app.listener_port, e);
			exit(99);
		}
	};
}



// main, currently testing app
fn main() {
	// create app
	let mut app = App::new(
		80,
		"127.0.0.1".to_string()
	);

	// register some test paths (GET /home; GET /rome;)
	app.reg_path(
		HttpMethod::GET,
		"/home".to_string(),
		Box::new(
			|| {
				return "<html><body><h1>hi</h1></body></html>".to_string();
			}
		)
	);
	app.reg_path(
		HttpMethod::GET,
		"/rome".to_string(),
		Box::new(
			|| {
				return "ON TO ROME SOLIDIER".to_string();
			}
		)
	);

	// start listening
	app.listen();
}