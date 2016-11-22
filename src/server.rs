// TODO: Implement a reader and writer from the client / server relationship.
// Then try and get clients to talk to everyone -> specific clients.
// Once that's done, move on to formatting messages via tokio::Encode / Decode.
// Example TCP

#[allow(unused_imports)]
use std::net::SocketAddr;
use std::collections::{HashMap};
use std::rc::Rc;
use std::cell::RefCell;

use std::io::{Error as IoError, ErrorKind};

use futures::AsyncSink;
use futures::Future;
use futures::stream::{Stream};
use futures::sink::{Sink};
use futures::sync::*;

use tokio_core::io::{Io,
};
use tokio_core::net::{TcpListener};
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
        let clients = self.clients.clone();
        let sender = self.channel.0.clone();

        println!("Chat service running at port : {}", self.addr.port());

        // For each incoming client connection at address 'addr'
        let server = socket.incoming().for_each(|(stream, addr)| {

            println!("Connected to client {}", addr);

            let mut server_sender = sender.clone();

            // reader   -> sender
            // receiver -> writer
            let (mut writer, reader) = TctClient::new(stream, addr)
                .framed(DataParser).split();
            let (sender, receiver) = mpsc::unbounded();

            clients.borrow_mut().insert(addr.port(), sender);

            let clients_inner = clients.clone();

            // Every message received over the stream, from client
            let reader = reader.for_each(move |msg: Data| {
                println!("Read made for {}", addr);
                if let Some(id) = msg.id() {
                    clients_inner.borrow_mut().get_mut(&id)
                        .unwrap_or(&mut server_sender)
                        .send(msg)
                        .or_else(
                            |err| Err(IoError::new(ErrorKind::Other, err)))
                    //Ok(())
                } else {
                    panic!("Client reported error")
                }
            }).map_err(|_| ());

            let receiver = receiver.for_each(move |mut msg| {
                println!("Writing message to {}", addr);
                let response = match writer.start_send(msg) { // handle it like 'send'
                    Ok(AsyncSink::Ready) => Ok(()),
                    Ok(AsyncSink::NotReady(balls)) => panic!("failed to send"),
                    Err(err) => Err(())
                };
                writer.poll_complete();
                response
            });

            //let clients = self.clients.clone();
            //let connection = receiver.map(|_| ()).select(reader.map(|_| ()));
            handle.spawn(receiver);
            handle.spawn(reader);

            Ok(())
        });
        self.core.run(server).unwrap();
    }
}

