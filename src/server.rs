// TODO: Implement a reader and writer from the client / server relationship.
// Then try and get clients to talk to everyone -> specific clients.
// Once that's done, move on to formatting messages via tokio::Encode / Decode.
// Example TCP
extern crate futures; // There is no reason why this should need to be here... 

use std::net::SocketAddr;
use std::io::{Error, Write, Read};
use std::str::{from_utf8};
use std::collections::{HashMap};
use std::rc::Rc;
use std::cell::RefCell;

use futures::{Future};
use futures::stream::Stream;

use tokio_core::io::{Io, ReadHalf, WriteHalf};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor::{Core, Handle};
use tokio_core::channel::{Sender, Receiver};

use client::TctClient;

type UserID = u16; // right now we'll just id by port number for ease

struct User {
    live_address: Vec<SocketAddr>,

}

pub struct TctServer {
    clients: Rc<RefCell<HashMap<UserID, Client>>>,
    core: Core,
    addr: SocketAddr,
}

/// Design choice: 'Client' struct will pertain to the struct which allows for
/// a server to 'write' to it's clients.
struct Client(WriteHalf<TcpStream>);

impl TctServer {

    pub fn new(addr: SocketAddr) -> TctServer {
        TctServer {
            core: Core::new().unwrap(),
            addr: addr,
            clients: Rc::new(RefCell::new(HashMap::new()))
        }
    }

    pub fn run(&mut self) {
        let socket =
            TcpListener::bind(&self.addr, &self.core.handle().clone()).unwrap();

        socket.incoming().for_each(|(stream, addr)| {
            let (reader, writer) = stream.split();

            self.clients.borrow_mut()
                .insert(addr.port(), Client(writer));

            //let clients_inner = self.clients.clone();

            Ok(())
        });
    }
}

