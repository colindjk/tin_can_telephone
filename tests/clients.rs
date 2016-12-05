#![feature(proc_macro, plugin, custom_attribute, custom_derive, plugin)]
#![plugin(serde_derive)]

extern crate tin_can_telephone as tct;

use std::net::{TcpStream as StdTcpStream, SocketAddr};
use std::io::{
    //Read, 
    Write};

#[test]
fn login() {
    let addr = "127.0.0.1:3000".to_string().parse::<SocketAddr>().unwrap();

    let mut tcp_a = match StdTcpStream::connect(&addr) {
        Ok(stream) => stream,
        Err(_) => panic!("Error, server not available, \
                         try running 'cargo run --example server'"),
    };

    let mut tcp_b = match StdTcpStream::connect(&addr) {
        Ok(stream) => stream,
        Err(_) => panic!("Error, server not available, \
                         try running 'cargo run --example server'"),
    };

    let login_a = "\"LoginCredentials\"{\"user\":\"login_a\"}\n".as_bytes();
    let login_b = "\"LoginCredentials\"{\"user\":\"login_b\"}\n".as_bytes();

    tcp_a.write(login_a).unwrap();
    tcp_a.flush().unwrap();
    tcp_b.write(login_b).unwrap();
    tcp_b.flush().unwrap();

    println!("Verify logins via server log, this point means succesful connection");
}

