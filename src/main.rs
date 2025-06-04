// imports
use mini_backend_framework::App;
use mini_backend_framework::HttpMethod;


// main, currently testing app
fn main() {
	// create app
	let mut app = App::new(
		3000,
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