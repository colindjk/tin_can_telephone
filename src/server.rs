// Example TCP

// EventLoop, EventSet, Io,
use mio::*;
use mio::tcp::*;

use std::io::Read;
use std::io::Write;

/// Central state of the program, managing connections etc.
pub struct TinServer {
    //pub  
}

pub fn run_echo() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    for buf_stream in listener.incoming().map(|stream| BufferedStream::new(stream)) {
        match buf_stream {
            Err(e) => { println!("failed: {}", e) }
            Ok(stream) => {
                println!("Startin' a connect!");
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
        }
    }
}

fn handle_client(mut stream: BufStream<TcpStream>) {
    let mut buf;
    stream.write("Hello client, please insert JSON into me.");

    loop {
        // clear out the buffer so we don't send garbage
        buf = [0; 4096];
        let _ = match stream.read(&mut buf) {
            Err(e) => panic!("Got an error: {}", e),
            Ok(m) => {
                if m == 0 {
                    // we've got an EOF
                    println!("Finished reading from client.");
                    break;
                }
                println!("Read a buffer.");
                m
            },
        };

        // take JSON.
        //if ()

        // Response of some sort

        match stream.write("Oh hai\n".to_string().as_bytes()) {
            Err(_) => break,
            Ok(_) => {

                continue
            }, // this is where a history would go.
        }
    }
}

