use crate::helpers::{request_handler::handle_incoming_stream, setup_listener::setup_listener};
use std::net::TcpListener;
use crate::HttpMethod;
use std::collections::HashMap;



// Represents a web application with configurable listener settings and route handlers.
pub struct App {
	pub listener_port: u16,
	pub listener_base_address: String,
	pub routes: HashMap<String, HashMap<String, Box<dyn Fn() -> String>>>,
	pub buffer_size: u64,
}



// actual functional implementation
impl App {
	// constructor function
	pub fn new(listener_port: u16, listener_base_address: String) -> Self {
		Self {
			listener_port,
			listener_base_address,
			buffer_size: 262144,
			routes: HashMap::new(),
		}
	}


	// function to register handlers for a specific route using a specific method
	pub fn reg_path(&mut self, method: HttpMethod, path: String, handler_function: Box<dyn Fn() -> String>) {
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
	pub fn listen(&mut self) {
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