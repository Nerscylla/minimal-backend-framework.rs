use mini_backend_framework::builders::request::Request;
// imports
use mini_backend_framework::App;
use mini_backend_framework::HttpMethod;

// main, currently testing app
fn main() {
    // create app
    let mut app = App::new(3000, "127.0.0.1".to_string());

    // register some test paths (GET /home; GET /rome;)
    app.reg_path(
        HttpMethod::GET,
        "/home".to_string(),
        Box::new(|_| {
            return "<html><body><h1>hi</h1></body></html>".to_string();
        }),
    );
    app.reg_path(
        HttpMethod::GET,
        "/rome".to_string(),
        Box::new(|req: Request| {
            return format!("ON TO ROME SOLIDIER {}", req.method);
        }),
    );

    app.reg_path(
        HttpMethod::GET,
        "/test/path".to_string(),
        Box::new(handler_test),
    );

    // start listening
    app.listen();
}

fn handler_test(req: Request) -> String {
    format!(
        "test path. Request stuff:\nMethod: {},\nPath: {},\nHeaders: {:?}, \nBody: {}",
        req.method, req.path, req.headers, req.body
    )
}
