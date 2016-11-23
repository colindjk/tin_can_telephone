// This file will include the Data (struct or enum) which will represent the
// different pieces of data which can be passed via HTTP style Request / Response
pub type UserID = u16; // right now we'll just id by port number for ease

use std::io;
//use std::result::

use tokio_core::io::{
    Codec, EasyBuf
};
//use tokio_core::io::frame::*;

use json::ser::{to_vec};
use json::de::{from_slice};

/// -- Global Constants --
static DELIMITER : u8 = b'\n' as u8;

/// The immutable struct which is passed between threads etc in order
/// to send and receive messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Data {
    Message { // Regular old message to UserID
        id: UserID,
        msg: String
    },    
    Request { id: UserID },         // friend request to user
    GroupRequest { id: UserID },    // Group invite request
    UserInfo {
        id: UserID,
        first_name: String,
        last_name: String,
    },

    Error(String),                  // Some sort of error?
    EOF,
}

impl Data {
    /// Optionally returns the ID the message should be sent to.
    pub fn id(&self) -> Option<UserID> {
        match *self {
            Data::Message{ id, .. }     => Some(id.clone()),
            Data::Request{ id }         => Some(id.clone()),
            Data::GroupRequest{ id }    => Some(id.clone()),
            Data::Error(_)              => None,
            _ => panic!("Unimplemented")
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
        -> Result<Option<Self::In>, io::Error>
    {
        println!("Decoding {}", buf.len());
        match buf.as_slice().iter().position(|&b| b == DELIMITER) {
            Some(index) => {
                println!("Decoding {}", index);
                let object_buf : EasyBuf = buf.drain_to(index + 1).into();
                Ok(Some(from_slice(object_buf.as_slice()).unwrap()))
            }
            None => Ok(None)
        }
    }

    /// Fills the buffer with the consumed 'Out' message.
    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        println!("Encoding buffer");
        if let Ok(mut json) = to_vec(&msg) {
            buf.append(&mut json); Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other, "Failed to encode object".to_string()))
        }

    }

    /// Reads until the end of stream, 
    fn decode_eof(&mut self, buf: &mut EasyBuf) -> Result<Self::In, io::Error> {
        println!("Stopped reading from client {}", buf.len());
        Ok(Data::EOF)
    }
}

