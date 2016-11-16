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
use std::result::Result;

use futures::{Future};
use futures::stream::Stream;

use tokio_core::io::{Io, ReadHalf, WriteHalf};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor::{Core, Handle};
use tokio_core::channel::{channel, Sender, Receiver};

use client::TctClient;
use data::{Data, UserID};

//pub struct ClientSocket(TctClient, (Sender<T>, Receiver<T>));

pub struct TctServer {
    clients: Rc<RefCell<HashMap<UserID, Sender<Data>>>>,
    channel: (Sender<Data>, Receiver<Data>),
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
            channel: channel(&core.handle()).unwrap(),
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
        let server_sender = self.channel.0.clone();

        socket.incoming().for_each(|(stream, addr)| {
            // Extract read (stream) / write (sink), 'write' to 'sender'.
            let socket = TctClient::new(stream, addr).framed::<Data, Data>();
            let (sender, receiver) = channel(&handle).unwrap();

            self.clients.borrow_mut().insert(addr.clone(), sender);

            let clients_inner = self.clients.clone();

            // Now this is a bit funky, but it's really just piping.
            // Messages get sent to online user, if user not found, sent to 
            // server to be handled (check DB, if no -> send error).
            let socket = socket.for_each(|msg: Data| {
                // How about just taking a random client for testing?
                // TODO: Unit test for the 'socket.for_each' functionality.
                if let Some(id) = msg.id() {
                    clients_inner.borrow().get(&id)
                                 .unwrap_or(&server_sender).send(msg); // panic?
                } else {
                    panic!("What do for error from client?");
                }
                Ok(())
            });
            // TODO what to do?

            //.and_then(|_| { // param is Ok(())
                //clients_inner.borrow_mut().remove(&addr);
            //});

            Ok(())
        });
    }
}

