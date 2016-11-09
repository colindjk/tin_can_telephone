extern crate mio;

//use mio::deprecated::*;
//use mio::{Token, EventSet};
//use mio::{IoDesc, IoHandle, ReadHint, Interest, PollOpt};
use mio::*;
use mio::tcp::{TcpListener, TcpStream};

const SERVER : Token = Token(0);
const CLIENT : Token = Token(1);

struct MyHandler(TcpListener);

impl Handler for MyHandler {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self,
             event_loop: &mut EventLoop<MyHandler>,
             token: Token,
             _: EventSet)
    {
        match token {
            SERVER => {
                let MyHandler(ref mut server) = *self;
                // Accept and drop?
                let _ = server.accept();
            }
            CLIENT => {
                event_loop.shutdown();
            }
            _ => panic!("Something bad happened.")
        }
    }
}

fn main() {

    let addr = "127.0.0.1:8888".parse().unwrap();

    // Setup the server socket
    let server = TcpListener::bind(&addr).unwrap();

    // Create an event loop
    let mut event_loop = EventLoop::new().unwrap();

    // Start listening for incoming connections
    event_loop.register(
        &server /* TcpListener */,
        SERVER  /* Token */,
        EventSet::readable(),
        PollOpt::edge()
    ).unwrap();

    // Setup the client socket
    let sock = TcpStream::connect(&addr).unwrap();

    // Register the socket
    event_loop.register(&sock, CLIENT, EventSet::readable(),
                        PollOpt::edge()).unwrap();


    event_loop.run(&mut MyHandler(server)).unwrap();

}
