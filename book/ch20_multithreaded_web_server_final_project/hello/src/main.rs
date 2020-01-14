use std::{fs, thread};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use hello::ThreadPool;

const LOG_ALL: bool = false;
const LOG_REQUESTS: bool = LOG_ALL || false;
const LOG_RESPONSES: bool = LOG_ALL || false;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
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

    let get = b"GET / ";
    let sleep = b"GET /sleep ";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_millis(100));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}\r\n\r\n{}", status_line, contents);
    let written = stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    if LOG_RESPONSES {
        println!();
        println!("-----====[ Response ]====-----");
        println!("{}", response);
        println!("Written {} bytes to response", written);
    }
}
