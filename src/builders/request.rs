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
    pub fn new(raw: String) -> Result<Request, String> {
        let request_seperator = SeperatedRawRequest::new(&raw);
        let first_line = match SplitFirstLine::new(request_seperator.start_line.clone()) {
            Some(first_line) => first_line,
            None => return Err("Invalid request start line".to_string()),
        };
        Ok(Self {
            raw: raw,
            method: first_line.method,
            path: first_line.path,
            split_path: first_line.split_path,
            headers: request_seperator.headers,
            body: request_seperator.body,
        })
    }
}

struct SeperatedRawRequest {
    start_line: String,
    headers: HashMap<String, String>,
    body: String,
}

impl SeperatedRawRequest {
    fn new(raw: &String) -> SeperatedRawRequest {
        let request_lines: Vec<&str> = raw.lines().collect();

        let (start_line, rest) = match request_lines.split_first() {
            Some((first, _rest)) => (first.to_string(), _rest),
            None => {
                eprintln!("Empty request received");
                (String::new(), &[][..])
            }
        };

        let mut headers = HashMap::<String, String>::new();
        let mut raw_body = String::new();
        let mut in_headers = true;

        for line in rest {
            if in_headers {
                if line.is_empty() {
                    in_headers = false;
                } else {
                    if let Some((key, value)) = line.split_once(':') {
                        headers.insert(key.trim().to_string(), value.trim().to_string());
                    }
                }
            } else {
                if !raw_body.is_empty() {
                    raw_body.push('\n');
                }
                raw_body.push_str(line);
            }
        }
        Self {
            start_line: start_line,
            headers: headers,
            body: raw_body,
        }
    }
}

struct SplitFirstLine {
    pub method: HttpMethod,
    pub path: String,
    pub split_path: Vec<String>,
}

impl SplitFirstLine {
    fn new(raw_first_line: String) -> Option<SplitFirstLine> {
        let split_first_line: Vec<String> =
            raw_first_line.split(' ').map(|s| s.to_string()).collect();
        // extract method and path from request
        if split_first_line.len() != 3 {
            return None;
        }
        let method: HttpMethod = HttpMethod::from_string(split_first_line[0].to_string());
        let mut path = split_first_line[1].to_string();
        // Remove trailing slash if present (but not for root "/")
        while path.len() > 1 && path.ends_with('/') {
            path = (&path[..path.len() - 1]).to_string();
        }
        let split_path: Vec<String> = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        Some(SplitFirstLine {
            method,
            path: path.to_string(),
            split_path,
        })
    }
}
