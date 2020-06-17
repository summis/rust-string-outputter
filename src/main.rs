use std::{thread, time};
use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use chrono::prelude::*;

fn rand_string() -> String {
    return thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();
}

// Server code from https://gist.github.com/mjohnsullivan/e5182707caf0a9dbdf2d
fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8 ;4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            println!("{}", req_str);
            },
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_write(mut stream: TcpStream) {
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
    match stream.write(response) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream) {
    handle_read(&stream);
    handle_write(stream);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let hash = rand_string();
    println!("Listening for connections on port {}", 8080);

    // Start separate thread for loop. `move` allows using hash inside thread.
    thread::spawn(move || {
        loop {
            let now = Utc::now();
            println!("{} {}", now, hash);
            thread::sleep(time::Duration::from_secs(5));
        }
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
