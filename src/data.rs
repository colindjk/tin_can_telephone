// This file will include the Data (struct or enum) which will represent the
// different pieces of data which can be passed via HTTP style Request / Response
pub type UserID = u16; // right now we'll just id by port number for ease

use std::net::SocketAddr;
use std::io::{Error as IoError, ErrorKind};
//use std::result::

use tokio_core::io::{Io, Codec, EasyBuf};
//use tokio_core::io::frame::*;

use json::ser::{to_vec};
use json::de::{from_slice};

pub type Message = Result<Data, String>;

/// The IMMUTABLE struct which is passed between threads etc in order
/// to send and receive messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Data {
    Message { // Regular old message to UserID
        id: UserID,
        msg: String
    },    
    Request { id: UserID },         // friend request to user
    GroupRequest { id: UserID },    // Group invite request
    Error(String),                  // Some sort of error?
}

impl Data {
    /// Optionally returns the ID the message should be sent to.
    pub fn id(&self) -> Option<UserID> {
        match *self {
            Data::Message{ id, .. }     => Some(id.clone()),
            Data::Request{ id }         => Some(id.clone()),
            Data::GroupRequest{ id }    => Some(id.clone()),
            Data::Error(_)              => None,
        }
    }
}

pub struct DataParser;

/// Decodability, this is where we decide on formatting.
impl Codec for DataParser {
    type In = Data;
    type Out = Data;

    /// Reads the EasyBuf.
    fn decode(&mut self, buf: &mut EasyBuf)
        -> Result<Option<Self::In>, IoError>
    {
        println!("Decoding");
        let len = buf.len();
        if let Ok(val) = from_slice(buf.as_ref()) {
            buf.drain_to(len);
            Ok(Some(val))
        } else {
            Err(IoError::new(ErrorKind::Other, "Decode error".to_string()))
        }
    }

    /// Fills the buffer with the consumed 'Out' message.
    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) {
        println!("Encoding buffer");
        if let Ok(mut json) = to_vec(&msg) {
            println!("Filling buffer");
            buf.append(&mut json);
        }
    }

    /// Reads until the end of stream, 
    fn decode_eof(&mut self, buf: &mut EasyBuf) -> Result<Self::In, IoError> {
        panic!("Unimplemented")
    }
}

