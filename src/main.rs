use std::{thread, time};
use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::sync::{Arc};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use chrono::prelude::*;

fn rand_string() -> String {
    return thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();
}

// Server code based on https://gist.github.com/mjohnsullivan/e5182707caf0a9dbdf2d
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

fn handle_write(mut stream: TcpStream, response: String) {
    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{}\r\n", response);
    match stream.write(response.as_bytes()) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream, response: String) {
    handle_read(&stream);
    handle_write(stream, response);
}

fn main() {
    // https://www.reddit.com/r/docker/comments/a8zhhl/rust_binary_listens_on_localhost3000_locally_not/
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    let hash = Arc::new(rand_string());

    // Start separate thread for loop. `move` allows using hash inside thread.
    // Cloning allows use of variable in many threads.
    {
        let hash = hash.clone().to_string();
        thread::spawn(move || {
            loop {
                let now = Utc::now();
                println!("{} {}", now, hash);
                thread::sleep(time::Duration::from_secs(5));
            }
        });
    }

    println!("Listening for connections on port {}", 8080);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let hash = hash.clone().to_string();
                thread::spawn(move || {
                    handle_client(stream, hash)
                });
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
