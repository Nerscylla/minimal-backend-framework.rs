use crate::{App, HttpMethod, builders::request::Request};
use std::{
    io::{Error, ErrorKind, Read, Write},
    net::TcpStream,
};

// hanlder function for streams coming in
pub fn handle_incoming_stream(app: &mut App, mut stream: TcpStream) {
    let raw_request = match get_request_string(app, &mut stream) {
        Ok(request) => request,
        Err(e) => {
            println!("Error getting Request: {}", e);
            return;
        }
    };

    let req: Request = match Request::new(raw_request) {
        Ok(req) => req,
        Err(e) => {
            eprintln!("Error generating Request struct: {}", e);
            return;
        }
    };

    // start building the response
    let response_text: String;

    // response function
    let response_function = app
        .routes
        .get(&HttpMethod::from_string(req.method.to_string()))
        .and_then(|method_map| method_map.get(&req.path));
    if let Some(handler) = response_function {
        // execute handler
        response_text = handler(req);
    } else {
        // no handler
        response_text = "404 not found".to_string();
    }

    // put response into HTTP response
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        response_text.len(),
        response_text
    );

    // send back HTTP over tcp stream
    match stream.write_all(response.as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to write response to stream: {}", e);
            return;
        }
    }
}

// helper function to go from tcp stream to http request text
fn get_request_string(app: &mut App, stream: &mut TcpStream) -> Result<String, Error> {
    // create buffer
    let buffer_size = app.buffer_size as usize;
    let mut buffer = vec![0u8; buffer_size];

    // Read up to buffer_size bytes
    let bytes_read = match stream.read(&mut buffer) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
            return Err(e);
        }
    };

    // If buffer is full, likely too large a request
    if bytes_read == buffer_size {
        eprintln!("Request too large, dropping connection");
        let response =
            "HTTP/1.1 413 Payload Too Large\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
        let _ = stream.write(response.as_bytes());
        return Err(Error::new(ErrorKind::Other, "Request too large"));
    }

    // move buffer into string and return
    Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
}
