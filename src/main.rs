#![feature(proc_macro, plugin, custom_attribute, custom_derive, plugin, test)]
#![plugin(serde_derive)]

// Imports:

// -- Parsing:
extern crate serde;
extern crate serde_xml;
#[macro_use] 
extern crate serde_derive;

// -- Logs:
extern crate env_logger;

// -- Network:
extern crate futures;
extern crate tokio_core;

// TODO: NEXT -> SERIALIZATION ULTIMATUM.
// TODO: mod server -> Client management, send / receive messages to clients
// TODO: mod client -> reads data given by a client, who writes back? (format? -> next)
// TODO: mod xmpp -> formatting -> implement XmlStream struct.
// TODO: mod server, client? -> Implement logging ('log') for the XML stream.

// TCP or UDP?
// TCP.

// Note on the copy(reader, writer) function, it's very simple.
// The function takes data given to reader and copies it over to writer,
// writing back whatever was written. The 'copy' might as well be called 'echo'.

// Also, in this case, when an incoming socket happens, we cannot handle the split
// without a future, since the connection will persist far longer than the
// 'for_each' iteration will, therefore it must be run asynchronously, to avoid
// only being able to handle one client at a time.

#[warn(unused_features, unused_imports)]
use std::net::SocketAddr;
use std::io::{Write, Read};
use std::str::{from_utf8};

use futures::Future;
use futures::stream::Stream;

use tokio_core::io::{copy, Io};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor::Core;

//use serde_json::*;

mod server;
mod client;
mod data;
//mod xmpp;

//use server::TctServer;

/// -- Global Constants --
static DELIMITER : u8 = b'\n' as u8;

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
            }).and_then(move |(r, w)| {
                //copy(r, Writer{ addr: addr.clone() }) // 'Connect' the streams.
                copy(r, w) // 'Connect' the streams.
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

