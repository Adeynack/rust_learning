use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

const LOG_REQUESTS: bool = true;
const LOG_RESPONSES: bool = true;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    if LOG_REQUESTS {
        println!();
        println!("-----====[ Request ]====-----");
        println!("{}", String::from_utf8_lossy(&buffer[..]));
    }

    let get = b"GET / HTTP/1.1\r\n";

    let response = if buffer.starts_with(get) {
        let contents = fs::read_to_string("hello.html").unwrap();
        format!("HTTP/1.1 200 OK\r\n\r\n{}", contents)
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("404.html").unwrap();
        format!("{}{}", status_line, contents)
    };

    if LOG_RESPONSES {
        println!();
        println!("-----====[ Response ]====-----");
        println!("{}", response);
    }

    let written = stream.write(response.as_bytes()).unwrap();
    println!("Written {} bytes to response", written);

    stream.flush().unwrap();
}
