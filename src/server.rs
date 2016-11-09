// Example TCP

// EventLoop, EventSet, Io,
use mio::*;
use mio::tcp::*;

use std::io::Read;
use std::io::Write;

const SERVER : Token = Token(0);
const CLIENT : Token = Token(1);

// Todo: How does our program work?
// 
//

/// Central state of the program, managing connections etc.
pub struct Server {

}


