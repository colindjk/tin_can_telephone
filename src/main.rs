//#![cfg_attr(feature = "serde_derive", feature(proc_macro))]

//#[cfg(feature = "serde_derive")]
//#[macro_use]
//extern crate serde_derive;

extern crate env_logger;
extern crate futures;
extern crate tokio_core;
//extern crate serde;
//extern crate serde_json;


// TCP or UDP?
// Can we get non-persistent TCP connections which don't have buffer stalls?
// Or should we work on UDP to make sure we have control over whether or not
// messages are sent / received?

// Note on the copy(reader, writer) function, it's very simple.
// The function takes data given to reader and copies it over to writer, writing back
// whatever was written. The 'copy' might as well be called 'echo'.

// Also, in this case, when an incoming socket happens, we cannot handle the split
// without a future, since the connection will persist far longer than the 'for_each'
// iteration will, therefore it must be run asynchronously, to avoid only being able
// to handle one client at a time.

use std::net::SocketAddr;
use std::io::{Write, Read};
use std::str::{from_utf8};

use futures::Future;
use futures::stream::Stream;

use tokio_core::io::{copy, Io};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor::Core;

//use serde_json::*;

//mod server;
//mod client;
mod data;
mod xmpp;

use data::Data;

struct Writer;

impl Write for Writer {
    fn write(&mut self, buf : &[u8]) -> Result<usize, std::io::Error> {
        print!("Recieved : {}", from_utf8(buf).unwrap().to_string());
        //let msg = from_utf8(buf).unwrap().to_string();
        //let val : Data = serde_json::from_slice(&buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

// We're gonna read some JSON
fn main() {
    let addr = "127.0.0.1:8080".to_string().parse::<SocketAddr>().unwrap();
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let socket = TcpListener::bind(&addr, &handle).unwrap();

    let core_loop = socket.incoming().for_each(|(stream, addr)| {
        let message =
            futures::lazy(move || {
                Ok(stream.split())
            }).and_then(|(r, _w)| {
                copy(r, Writer)
            }).map(move |amt| {
                println!("Said hello to client {} at {}!", amt, addr);
            }).map_err(|e| {
                panic!("Error: {}", e);
            });

        handle.spawn(message);

        Ok(())
    });

    core.run(core_loop).unwrap();
}

