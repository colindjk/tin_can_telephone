// TODO: Implement a reader and writer from the client / server relationship.
// Then try and get clients to talk to everyone -> specific clients.
// Once that's done, move on to formatting messages via tokio::Encode / Decode.
// Example TCP

#[allow(unused_imports)]
use std::net::SocketAddr;
use std::collections::{HashMap};
use std::rc::Rc;
use std::cell::RefCell;

use std::io;

use futures::AsyncSink;
use futures::sink::{Sink};
use futures::sync::mpsc;

use futures::future::{Future};
use futures::stream::{Stream};

use tokio_core::io::Io;
use tokio_core::net::{TcpListener};
use tokio_core::reactor::{Core};

use client::TctClient;
use stanza::{Stanza, StanzaCodec, UserID};

pub struct TctServer {
    clients: Rc<RefCell<HashMap<UserID, mpsc::UnboundedSender<Stanza>>>>,
    #[allow(dead_code)]
    groups: Rc<RefCell<HashMap<UserID, mpsc::UnboundedSender<Stanza>>>>,
    channel: (mpsc::UnboundedSender<Stanza>, mpsc::UnboundedReceiver<Stanza>),
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
            groups: Rc::new(RefCell::new(HashMap::new())),
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

            println!("Connected to client : {}", addr);

            let mut server_sender = sender.clone();

            // (writer, reader) == (sink, stream)
            // reader   -> sender
            // receiver -> writer
            let (mut writer, reader) = TctClient::new(stream, addr)
                .framed(StanzaCodec).split();
            let (sender, receiver) = mpsc::unbounded();

            let clients_inner = clients.clone();

            // Every message received over the stream, from client
            let reader = reader.into_future().map_err(|(err, _)| err).and_then(
                |(creds, stream)| {
                    if let Some(Stanza::LoginCredentials{ user, psw: _ }) = creds {
                        if user.len() == 0 {
                            println!("No username supplied. \
                                      Closing stream...");
                        } else {
                            println!("User {} logged in!", user);
                            clients_inner.borrow_mut().insert(user, sender);
                            // TODO: System of verification that a user is in the db.
                        }
                    } else if let Some(Stanza::Register{ user, psw: _ }) = creds {
                        println!("New user {} logged in!", user);
                        clients_inner.borrow_mut().insert(user, sender);
                    } else {
                        println!("No login credentials supplied from client. \
                                  Closing stream...");
                        // By not giving resources for sender into HashMap,
                        // data does not persist for this client, connection
                        // will then time-out.
                    }
                    stream.for_each(move |msg: Stanza| {
                        println!("Read made for"); // TODO: Give addr here
                        if let Some(to) = msg.to() {
                            clients_inner.borrow_mut().get_mut(&to)
                                .unwrap_or(&mut server_sender) // TODO: 
                                .send(msg)
                                .or_else(
                                    |err| Err(io::Error::new(
                                            io::ErrorKind::Other, err)))

                        } else { panic!("Client reported error") }
                    })
                }).map_err(|_| ());

            let receiver = receiver.for_each(move |msg| {
                println!("Writing message to {}", addr);
                let response = match writer.start_send(msg) { // handle it like 'send'
                    Ok(AsyncSink::Ready) => Ok(()),
                    Ok(AsyncSink::NotReady(_)) => panic!("failed to send"),
                    Err(_) => Err(())
                };
                writer.poll_complete().unwrap();
                response
            });

            //let clients = self.clients.clone();
            // TODO: 'select' combinator
            handle.spawn(receiver);
            handle.spawn(reader);

            Ok(())
        });
        self.core.run(server).unwrap();
    }
}

