// TODO: How to send / receive the things. And format.
use std::net::SocketAddr;
use std::io::{Error, Write, Read};
use std::str::{from_utf8};
use std::collections::{HashMap};

use futures::{Future};
use futures::stream::Stream;

use tokio_core::net::{TcpStream};
use tokio_core::reactor::{Core, Handle};


/// Current implementation of our message interpreter, ideally will be the
/// XMPP stream.
/// Remember, there is one connection per client. Regular XMPP chat's would
/// allow for multiple simultaneous connections to the same stream, not sure
/// if we want to do that.
pub struct TctClient {
    stream: TcpStream,
    addr: SocketAddr
}

impl TctClient {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> TctClient {
        TctClient {
            stream: stream,
            addr: addr
        }
    }
}

impl Write for TctClient {
    fn write(&mut self, buf : &[u8]) -> Result<usize, Error> {
        print!("Client : {}", from_utf8(buf).unwrap().to_string());
        //let msg = from_utf8(buf).unwrap().to_string();
        //let val : Data = serde_json::from_slice(&buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl Read for TctClient {
    fn read(&mut self, buf : &mut [u8]) -> Result<usize, Error> {
        self.stream.read(buf);
        Ok(buf.len())
    }
}
