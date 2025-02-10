mod router;

use std::{
    io::{Read, Write, BufRead, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};

#[allow(unused_imports)]

struct Request {
    method: String,
    path: Vec<String>,
    user_agent: String,
    content_type: Option<String>,
    content_length: Option<u8>,
    content_body: Option<String>,
}

// ===
// Reading
// ===

fn read_stream(stream: &TcpStream) -> (Vec<u8>, Vec<String>) {
    let mut buf_reader = BufReader::new(stream);
    let mut headers = Vec::new();
    let mut content_length = 0;

    for line in buf_reader.by_ref().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        if line.starts_with("Content-Length:") {
            if let Some(len) = line.split_whitespace().nth(1) {
                content_length = len.parse::<usize>().unwrap_or(0);
            }
        }
        headers.push(line);
    }

    // Read the body based on Content-Length
    let mut body = vec![0; content_length];

    if content_length > 0 {
        buf_reader.read_exact(&mut body).unwrap();
    }

    (body, headers)
}

// ===
// Parsing
// ===

fn parse_headers(request: Vec<String>) -> Request {
    let request_line = &request[0];
    let parts: Vec<&str> = request_line.split_whitespace().collect();

    println!("--- Request ---");
    println!("{:#?}", &request);
    println!("---------------");

    let method = parts[0].to_string();
    let mut path: Vec<String> = parts[1]
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    if path.is_empty() {
        path.push("".to_string());
    }

    let mut user_agent = "NoAgentProvided".to_string();
    let mut content_type = None;
    let mut content_length = None;

    // Process headers
    for line in request.iter().skip(1) {
        if let Some((key, value)) = line.split_once(':') {
            let key_lower = key.trim().to_lowercase();
            let value_clean = value.trim().to_string();

            match key_lower.as_str() {
                "user-agent" => user_agent = value_clean,
                "content-type" => content_type = Some(value_clean),
                "content-length" => {
                    if let Ok(len) = value_clean.parse::<u32>() {
                        content_length = Some(len);
                    }
                }
                _ => {} // Ignore other headers
            }
        }
    }

    Request {
        method,
        path,
        user_agent,
        content_type,
        content_length: content_length.map(|l| l as u8), // Convert to u8 if needed
        content_body: None,
    }
}

fn handle_connection(stream: TcpStream) {
    let (body, headers) = read_stream(&stream);
    let parsed_headers = parse_headers(headers);
    let mut request: Request = parsed_headers;

    // Parse body to string
    if body.len() > 0 {
        let parsed_body = String::from_utf8_lossy(&body).to_string();
        request.content_body = Option::from(parsed_body);
    }

    router::function(&stream, &request);
}

fn main() {
    println!("Server started");
    let listener = TcpListener::bind("127.0.0.1:4221").expect("Could not bind.");
    println!("Listening on: {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        match stream {
            // TODO: add thread pool
            Ok(stream) => {
                thread::spawn(|| {
                    println!("\n===");
                    println!("Connection: {}\n", stream.peer_addr().unwrap());
                    handle_connection(stream);
                    println!("===\n");
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

