#![feature(proc_macro, plugin, custom_attribute, custom_derive, plugin, test)]
#![plugin(serde_derive)]

// Imports:

// -- Parsing:
extern crate serde;
extern crate serde_xml as xml;
extern crate serde_json as json;
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

mod server;
mod client;
mod data;

use server::TctServer;

/// -- Global Constants --
//static DELIMITER : u8 = b'\n' as u8;

// We're gonna read some JSON
fn main() {
    let addr = "127.0.0.1:8080".to_string().parse::<SocketAddr>().unwrap();

    let mut server = TctServer::new(addr);

    server.run();
}

