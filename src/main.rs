extern crate env_logger;
extern crate futures;
extern crate tokio_core;

// TCP or UDP?
// Can we get non-persistent TCP connections which don't have buffer stalls?
// Or should we work on UDP to make sure we have control over whether or not
// messages are sent / received?

// Note on the copy(reader, writer) function, it's very simple.
// The function takes data given to reader and copies it over to writer, writing back
// whatever was written. The 'copy' might as well be called 'echo'.

// Also, in this case, when an incoming socket happens, we cannot handle the split
// without a future, since the connection will persist far longer than the 'for_each'
// iteration will, therefore it must be run asynchronously, to avoid only being able
// to handle one client at a time.

use std::net::SocketAddr;

use futures::Future;
use futures::stream::Stream;
use tokio_core::io::{copy, Io};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;

fn main() {
    let addr = "127.0.0.1:8080".to_string().parse::<SocketAddr>().unwrap();

    let mut core_loop = Core::new().unwrap();
    let handle = core_loop.handle();

    // initiate a tcp listener (server's role)
    let socket = TcpListener::bind(&addr, &handle).unwrap();

    println!("I'm listening on {}", addr);

    let done = socket.incoming().for_each(move |(socket, addr)| {

        // Copy's all bytes from 'reader' (from the client), to 'writer' (to the client).
        //let io_pair = futures::lazy(|| Ok(socket.split()));
        //let amt = io_pair.and_then(|(reader, writer)| { copy(reader, writer) });

        println!("I'm talking on {}", addr);

        //let msg = amt.map(move |amt| {
            //println!("Wrote {} bytes to {}", amt, addr);
        //}).map_err(|e| {
            //panic!("Error: {}", e);
        //});

        let msg = futures::lazy(|| Ok(socket.split()))
            .and_then(|(r, w)| copy(r, w))
            .map(move |amt| {
                println!("Wrote {} bytes to {}", amt, addr);
            }).map_err(|e| {
            panic!("Error: {}", e);
        });

        handle.spawn(msg);

        Ok(())
    });

    core_loop.run(done).unwrap();
}

