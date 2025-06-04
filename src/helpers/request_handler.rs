use std::{io::{Error, ErrorKind, Read, Write}, net::TcpStream};
use crate::App;



// hanlder function for streams coming in
pub fn handle_incoming_stream(app: &mut App, mut stream: TcpStream) {
    let request = match get_request_string(app, &mut stream) {
        Ok(request) => request,
        Err(e) => {
            println!("Error getting Request: {}", e);
            return;
        }
    };


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
    let method = if split_first_line.len() == 3 { &split_first_line[0] } else { return };
    let mut path = if split_first_line.len() == 3 { split_first_line[1].as_str() } else { return };
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
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
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
        let response = "HTTP/1.1 413 Payload Too Large\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
        let _ = stream.write(response.as_bytes());
        return Err(Error::new(ErrorKind::Other, "Request too large"));
    }

    // move buffer into string and return
    Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
}