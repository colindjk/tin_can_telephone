// Example TCP

use std::net::SocketAddr;
use std::io::{Write, Read};
use std::str::{from_utf8};

use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;

type Username = String;

struct User {
    live_address: Vec<SocketAddr>,

}

struct TctServer {
    socket: TcpListener,
    online: HashMap<Username, User>
}

/// Current implementation of our message interpreter, ideally will be the
/// XMPP stream.
/// Remember, there is one connection per client. Regular XMPP chat's would
/// allow for multiple simultaneous connections to the same stream, not sure
/// if we want to do that.
struct ClientStream {
    stream: TcpStream,
    addr: SocketAddr,
}

impl Write for Writer {
    fn write(&mut self, buf : &[u8]) -> Result<usize, std::io::Error> {
        print!("{} : {}", self.addr.port(), from_utf8(buf).unwrap().to_string());
        //let msg = from_utf8(buf).unwrap().to_string();
        //let val : Data = serde_json::from_slice(&buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

/// Central state of the program, managing connections etc.
pub struct Server {
    socket: TcpListener,
    //connections: HashMap<Token, Client>,
    //users: HashMap<Token, Client>, // for now we'll just have a hashmap of online users.
}


