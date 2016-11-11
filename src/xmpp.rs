
use std::net::SocketAddr;
use std::io::{Write, Read};
use std::str::{from_utf8};

use futures::Future;
use futures::stream::Stream;

use tokio_core::io::{copy, Io};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor::Core;

struct XmlStream {
    stream: TcpStream,
    id: str,
    host: str,
}

