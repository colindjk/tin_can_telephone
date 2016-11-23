// TODO: How to send / receive the things. And format.
use std::net::SocketAddr;
use std::io;
//use std::str::{from_utf8};
//use std::collections::{HashMap};

//use futures::{Future};
//use futures::stream::Stream;
use futures::future::Future;

use tokio_core::net::{TcpStream};
use tokio_core::reactor::{
    //Core,
    Handle
};
use tokio_core::io::{ // Organized the imports to give a visual rep
    Io,
    //ReadHalf, WriteHalf,
};

//use data;

/// Current implementation of our message interpreter, ideally will be the
/// XMPP stream.
/// Remember, there is one connection per client. Regular XMPP chat's would
/// allow for multiple simultaneous connections to the same stream, not sure
/// if we want to do that.
pub struct TctClient {
    stream: TcpStream,

    #[allow(dead_code)]
    addr: SocketAddr
}

/// Implementation for the client
/// TODO: Handle signals?
impl TctClient {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> TctClient {
        TctClient {
            stream: stream,
            addr: addr
        }
    }

    /// To be used for a stand-alone client.
    #[allow(dead_code)]
    pub fn new_client(addr: &SocketAddr, core_handle: &Handle)
        -> Result<TctClient, io::Error>
    {
        Ok( TctClient {
            stream: TcpStream::connect(addr, core_handle).wait()?, // wait?
            addr: addr.clone()
        })
    }

}

/// Trait to be used by the server to consume and use the reading / writing of
/// this client via sendable objects (data.rs?).
impl Io for TctClient {

}

impl io::Write for TctClient {
    fn write(&mut self, buf : &[u8]) -> Result<usize, io::Error> {
        self.stream.write(buf)
    }
    fn flush(&mut self) -> Result<(), io::Error> {
        self.stream.flush()
    }
}

impl io::Read for TctClient {
    fn read(&mut self, buf : &mut [u8]) -> Result<usize, io::Error> {
        self.stream.read(buf)
    }
}
