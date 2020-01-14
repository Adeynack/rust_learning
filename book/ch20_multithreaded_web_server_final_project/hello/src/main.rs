use std::{fs, thread};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::time::Duration;

use hello::ThreadPool;

const LOG_ALL: bool = false;
const LOG_REQUESTS: bool = LOG_ALL || false;
const LOG_RESPONSES: bool = LOG_ALL || false;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    let (main_loop_sender, main_loop_receiver) = mpsc::channel::<MainLoopMessage>();

    let main_loop_sender_for_listen = main_loop_sender.clone();
    thread::spawn(move || {
        listen(listener, main_loop_sender_for_listen);
    });

    let main_loop_sender_for_ctrlc = main_loop_sender.clone();
    ctrlc::set_handler(move || {
        println!("Ctrl-C Detected. Sending Terminate signal.");
        main_loop_sender_for_ctrlc.send(MainLoopMessage::Terminate).unwrap();
    }).unwrap();

    loop {
        match main_loop_receiver.recv().unwrap() {
            MainLoopMessage::Terminate => {
                println!("Terminate signal received. Breaking main loop.");
                break;
            }
            MainLoopMessage::NewConnection(stream) => {
                pool.execute(|| {
                    handle_connection(stream);
                });
            }
        }
    }

    println!("Shutting down.");
}

enum MainLoopMessage {
    Terminate,
    NewConnection(TcpStream),
}

fn listen(listener: TcpListener, sender: Sender<MainLoopMessage>) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                sender.send(MainLoopMessage::NewConnection(stream)).unwrap();
            }
            Err(err) => {
                println!("Error opening connection: {}", err);
                break;
            }
        }
    }
    println!("TcpListener has no more incoming connection.");
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
        thread::sleep(Duration::from_millis(500));
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
