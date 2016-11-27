// TODO: How to send / receive the things. And format.
use std::net::SocketAddr;
use std::io;

use futures::future::Future;

use tokio_core::net::{TcpStream};
use tokio_core::reactor;
use tokio_core::io::{ // Organized the imports to give a visual rep
    Io,
};

use stanza;

/// Current implementation of our message interpreter, ideally will be the
/// XMPP stream.
/// Remember, there is one connection per client. Regular XMPP chat's would
/// allow for multiple simultaneous connections to the same stream, not sure
/// if we want to do that.
pub struct TctClient {
    user: Option<stanza::UserID>, // A connection can be made before a login!
    stream: TcpStream,

    #[allow(dead_code)]
    addr: SocketAddr,
}

/// Implementation for the client
/// TODO: Handle signals?
impl TctClient {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> TctClient {
        TctClient {
            stream: stream,
            user: None,
            addr: addr,
        }
    }

    /// To be used for a stand-alone client.
    #[allow(dead_code)]
    pub fn new_client(addr: &SocketAddr, core_handle: &reactor::Handle)
        -> Result<TctClient, io::Error>
    {
        Ok( TctClient {
            stream: TcpStream::connect(addr, core_handle).wait()?, // wait?
            user: None,
            addr: addr.clone(),
        })
    }

    /// Read a line from std input, used for talking to someone, obviously.
    pub fn read_line(&self, buf: &mut String) {
        io::stdin().read_line(buf).unwrap();
    }

}

/// Trait to be used by the server to consume and use the reading / writing of
/// this client via sendable objects (stanza.rs?).
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

#[cfg(test)]
mod test {
    use super::TctClient;

    #[test]
    fn example_client() {

    }
}
