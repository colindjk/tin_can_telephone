// Example TCP

use std::net::{TcpListener, TcpStream};
use std::thread;

use std::io::Read;
use std::io::Write;

fn handle_client(mut stream: TcpStream) {
    let mut buf;
    loop {
        // clear out the buffer so we don't send garbage
        buf = [0; 512];
        let _ = match stream.read(&mut buf) {
            Err(e) => panic!("Got an error: {}", e),
            Ok(m) => {
                if m == 0 {
                    // we've got an EOF
                    break;
                }
                m
            },
        };

        match stream.write(&buf) {
            Err(_) => break,
            Ok(_) => continue,
        }
    }
}

pub fn run_echo() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    for stream in listener.incoming() {
        match stream {
            Err(e) => { println!("failed: {}", e) }
            Ok(stream) => {
                println!("Startin' a connect!");
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
        }
    }
}

