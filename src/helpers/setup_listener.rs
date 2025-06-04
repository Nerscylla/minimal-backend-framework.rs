use std::process::exit;
use crate::App;
use std::net::TcpListener;



// helper function to set up the listener to prevent unreadable nesting
pub fn setup_listener(app: &mut App) -> TcpListener {
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