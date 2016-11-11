// Example TCP

// Todo: How does our program work?
// 
//
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;

/// Central state of the program, managing connections etc.
pub struct Server {
    socket: TcpListener,
    connections: HashMap<Token, Client>,
    users: HashMap<Token, Client>, // for now we'll just have a hashmap of online users.
}


