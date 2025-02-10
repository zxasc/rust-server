use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::net::TcpStream;
use crate::{Request};

type Handler = fn(stream: &TcpStream, _request: &Request);

fn handler_root(mut stream: &TcpStream, _request: &Request) {
    let response = b"HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response).expect("Failed to send response.");
}
fn handler_echo(mut stream: &TcpStream, request: &Request) {
    let response_string = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", request.path[1].len(), request.path[1]);
    stream.write_all(response_string.as_ref()).expect("Failed to send response.");
}
fn handler_user_agent(mut stream: &TcpStream, request: &Request) {
    let response_string = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", request.user_agent.len(), request.user_agent);
    stream.write_all(response_string.as_ref()).expect("Failed to send response.");
}

fn handler_files(mut stream: &TcpStream, request: &Request) {
    let dir_path = "/tmp/usercontent/";
    let file_path = format!("{}/{}", dir_path, request.path[1]);


    if request.method == "GET" {
        match fs::metadata(&file_path) {
            Ok(metadata) => {
                if metadata.is_file() {
                    let file_contents = fs::read(file_path).unwrap();
                    let content_len = file_contents.len();
                    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n", content_len);
                    stream.write_all(response.as_bytes()).expect("Failed to send response.");
                    stream.write_all(&*file_contents).expect("Failed to send response.");
                } else {
                    let response = b"HTTP/1.1 404 Not Found\r\n\r\n";
                    stream.write_all(response).expect("Failed to send response.");
                }
            }
            Err(_) => {
                let response = b"HTTP/1.1 404 Not Found\r\n\r\n";
                stream.write_all(response).expect("Failed to send response.");
            }
        }
    } else if request.method == "POST" {
        // handle post
        match fs::metadata(&dir_path) {
            Ok(_metadata) => {
                let mut file = File::create(file_path).unwrap();
                file.write_all(request.content_body.as_ref().unwrap().as_bytes()).unwrap();
                let response = b"HTTP/1.1 201 Created\r\n\r\n";
                stream.write_all(response).expect("Failed to send response.");
            }
            Err(_) => {
                let response = b"HTTP/1.1 404 Not Found\r\n\r\n";
                stream.write_all(response).expect("Failed to send response.");
            }
        }
    } else {
        // TODO: add handling for DELETE
        let response = b"HTTP/1.1 404 Not Found\r\n\r\n";
        stream.write_all(response).expect("Failed to send response.");
    }
}

fn handler_not_found(mut stream: &TcpStream, _request: &Request) {
    let response = b"HTTP/1.1 404 Not Found\r\n\r\n";
    stream.write_all(response).expect("Failed to send response.");
}
pub fn function(stream: &TcpStream, request: &Request) {

    let mut router: HashMap<String, Handler> = HashMap::new();
    router.insert("".to_string(), handler_root);
    router.insert("echo".to_string(), handler_echo);
    router.insert("user-agent".to_string(), handler_user_agent);
    router.insert("files".to_string(), handler_files);

    if let Some(handler) = router.get(&request.path[0]) {
        handler(stream, request);
    } else {
        handler_not_found(stream, request);
    }
}