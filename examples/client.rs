#![feature(proc_macro, plugin, custom_attribute, custom_derive, plugin)]
#![plugin(serde_derive)]

// -- Network:
extern crate futures;
extern crate tokio_core;

extern crate tin_can_telephone as tct;

use std::net::SocketAddr;

use tct::client::TctClient;

use tokio_core::reactor::Core;

// We're gonna read some JSON
fn main() {
    let core = Core::new().unwrap();
    let addr = "127.0.0.1:8080".to_string().parse::<SocketAddr>().unwrap();

    let mut client = TctClient::new_client(&addr, &core.handle()).unwrap();

    client.run();
}

