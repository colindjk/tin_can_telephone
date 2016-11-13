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

use tokio_core::io::{copy, Io};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor::{Core, Handle};

use client::TctClient;

type UserID = u16; // right now we'll just id by port number for ease

struct User {
    live_address: Vec<SocketAddr>,

}

pub struct TctServer {
    clients: Rc<RefCell<HashMap<UserID, TctClient>>>,
    core: Core,
    addr: SocketAddr,
}

impl TctServer {

    pub fn new(addr: SocketAddr) -> TctServer {
        TctServer {
            core: Core::new().unwrap(),
            addr: addr,
            clients: Rc::new(RefCell::new(HashMap::new()))
        }
    }

    fn run(&mut self) {
        let socket =
            TcpListener::bind(&self.addr, &self.core.handle().clone()).unwrap();
        socket.incoming().for_each(move |(stream, addr)| {
            self.clients.borrow_mut()
                .insert(addr.port(), TctClient::new(stream, addr));
            //let do_thing =
                //futures::lazy(move || {
                    //self.clients.insert(
                        //addr.port(),
                        //TctClient::new(stream, addr)
                    //);
                //}).and_then(move |(r, w)| {
                    //// copy(r, Writer{ addr: addr.clone() })
                    //// 'Connect' the streams.
                    //copy(r, w) // 'Connect' the streams.
                //}).map(move |amt| {
                    //println!("Said hello to client {} at {}!", amt, addr);
                //}).map_err(|e| {
                    //panic!("Error: {}", e);
                //});
            //self.handle.clone().spawn(do_thing);
            Ok(())
        });
    }
}

