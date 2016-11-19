// TODO: Implement a reader and writer from the client / server relationship.
// Then try and get clients to talk to everyone -> specific clients.
// Once that's done, move on to formatting messages via tokio::Encode / Decode.
// Example TCP

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
use std::result::Result;

use futures::{
    Future,
};
use futures::stream::{Stream};
use futures::sync::*;

use tokio_core::io::{Io, ReadHalf, WriteHalf};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor::{Core, Handle};
// tokio::channel is deprecated

use client::TctClient;
use data::{Data, DataParser, UserID};

pub struct TctServer {
    clients: Rc<RefCell<HashMap<UserID, mpsc::UnboundedSender<Data>>>>,
    channel: (mpsc::UnboundedSender<Data>, mpsc::UnboundedReceiver<Data>),
    core: Core,
    addr: SocketAddr,
}

// Ixnay on the private client, defeats the purpose and actually breaks the design
// in the first place lulz.

impl TctServer {
    /// Creates a new TctServer to be run.
    pub fn new(addr: SocketAddr) -> TctServer {
        let core = Core::new().unwrap();
        TctServer {
            // Odd way of doing this but yeah, core needs to be defined after
            // channel.
            channel: mpsc::unbounded(),
            core: core,
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

        // For each incoming client connection at address 'addr'
        socket.incoming().for_each(|(stream, addr)| {

            let server_sender = self.channel.0.clone();

            let socket = TctClient::new(stream, addr).framed(DataParser);
            let (sender, receiver) = mpsc::unbounded();

            self.clients.borrow_mut().insert(addr.clone(), sender);

            let clients_inner = self.clients.clone();

            // Now this is a bit funky, but it's really just piping.
            // Messages get sent to online user, if user not found, sent to 
            // server to be handled (check DB, if no -> send error).
            let socket = socket.for_each(|msg: Data| {
                let clients = clients_inner.clone();
                // How about just taking a random client for testing?
                // TODO: Unit test for the 'socket.for_each' functionality.
                if let Some(id) = msg.id() {
                    let mut clients = clients.borrow_mut();
                    clients.get(&id)
                                 .unwrap_or(&server_sender).send(msg); // panic?
                } else {
                    panic!("What do for error from client?");
                }
                Ok(())
            });
            // TODO what to do?

            Ok(())
        });
    }
}

