#![feature(proc_macro, plugin, custom_attribute, custom_derive, plugin)]
#![plugin(serde_derive)]

// -- Network:
extern crate futures;
extern crate tokio_core;

extern crate tin_can_telephone as tct;

use std::net::SocketAddr;

use tokio_core::reactor::Core;

use tct::server;

fn main() {
    let core = Core::new().unwrap();
    let addr = "127.0.0.1:3000".to_string().parse::<SocketAddr>().unwrap();

    let mut server = server::TctServer::new(addr);

    server.run();
}

