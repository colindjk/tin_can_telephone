// TODO: Implement a reader and writer from the client / server relationship.
// Then try and get clients to talk to everyone -> specific clients.
// Once that's done, move on to formatting messages via tokio::Encode / Decode.
// Example TCP
extern crate futures; // There is no reason why this should need to be here... 

#[allow(unused_imports)]
use std::net::SocketAddr;
use std::io::{
    Error,
    ErrorKind,
    Write,
    Read
};
use std::str::{from_utf8};
use std::collections::{HashMap};
use std::rc::Rc;
use std::cell::RefCell;

use futures::{Future};
use futures::stream::Stream;

use tokio_core::io::{Io, ReadHalf, WriteHalf};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor::{Core, Handle};
use tokio_core::channel::{channel, Sender, Receiver};

use client::TctClient;

type UserID = SocketAddr; // right now we'll just id by port number for ease

//pub struct ClientSocket(TctClient, (Sender<T>, Receiver<T>));

pub struct TctServer {
    clients: Rc<RefCell<HashMap<UserID, Sender<TcpStream>>>>,
    core: Core,
    addr: SocketAddr,
}

// Ixnay on the private client, defeats the purpose and actually breaks the design
// in the first place lulz.

impl TctServer {
    /// Creates a new TctServer to be run.
    pub fn new(addr: SocketAddr) -> TctServer {
        TctServer {
            core: Core::new().unwrap(),
            addr: addr,
            clients: Rc::new(RefCell::new(HashMap::new()))
        }
    }

    /// Execute the server, runs in foreground for any application...
    /// This is a temporary design to get something off the ground.
    pub fn run(&mut self) {
        let socket =
            TcpListener::bind(&self.addr, &self.core.handle().clone()).unwrap();
        let handle = self.core.handle();

        socket.incoming().for_each(|(stream, addr)| {
            let (sender, receiver) = channel(&handle).unwrap();
            self.clients.borrow_mut().insert(addr, sender);

            let client_socket = TctClient::new(stream, addr);
            let clients_inner = self.clients.clone();

            Ok(())
        });
    }
}

