// This file will include the Stanza (struct or enum) which will represent the
// different pieces of stanza which can be passed via HTTP style Request /
// Response.
pub type UserID = u16; // right now we'll just id by port number for ease

use std::io;

use tokio_core::io::{
    Codec, EasyBuf
};

use json::ser::{to_vec};
use json::de::{from_slice};

/// -- Global Constants --
static DELIMITER : u8 = b'\n' as u8;

/// The immutable struct which is passed between threads etc in order
/// to send and receive messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Stanza {
    Message { // Regular old message to UserID
        to: UserID,
        from: UserID,
        msg: String
    },    
    Request {
        to: UserID,
        from: UserID,
    },  // friend request to user
    GroupRequest {
        to: UserID,
        from: UserID,
    },    // Group invite request
    Response {
        to: UserID,
        from: UserID,
    },  // friend request to user
    GroupResponse {
        to: UserID,
        from: UserID,
    },    // Group invite request
    UserInfo {
        to: UserID,
        from: UserID,
        first_name: String,
        last_name: String,
    },
    LoginCredentials {
        from: UserID,
        password: String,
    },
    History(Vec<String>),

    Error(String),                  // Some sort of error?
    EOF,
}

impl Stanza {

    /// Optionally returns the ID the message should be sent to.
    pub fn to(&self) -> Option<UserID> {
        match *self {
            Stanza::Message{ to, .. }       => Some(to.clone()),
            Stanza::Request{ to, .. }       => Some(to.clone()),
            Stanza::GroupRequest{ to, .. }  => Some(to.clone()),
            Stanza::Error(_)                => None,
            _                               => panic!("Unimplemented")
        }
    }

    /// Optionally returns the ID the message should be sent to.
    pub fn from(&self) -> Option<UserID> {
        match *self {
            Stanza::Message{ from, .. }       => Some(from.clone()),
            Stanza::Request{ from, .. }       => Some(from.clone()),
            Stanza::GroupRequest{ from, .. }  => Some(from.clone()),
            Stanza::Error(_)                  => None,
            _ => panic!("Unimplemented")
        }
    }
}

pub struct StanzaCodec;

/// Decodability, this is where we decide on formatting.
impl Codec for StanzaCodec {
    type In = Stanza;
    type Out = Stanza;

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

//  /// Reads until the end of stream, 
//  fn decode_eof(&mut self, buf: &mut EasyBuf) -> Result<Self::In, io::Error> {
//      println!("Stopped reading from client {}", buf.len());
//      Ok(Stanza::EOF)
//  }
}

