// This file will include the Stanza (struct or enum) which will represent the
// different pieces of stanza which can be passed via HTTP style Request /
// Response.
pub type UserID = u16; // right now we'll just id by port number for ease
pub type TimeStamp = u64;

use std::io;
use std::collections::{HashMap};

use tokio_core::io::{
    Codec, EasyBuf
};

use json::ser::{to_vec};
use json::de::{from_slice};

use my;

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
    GroupMessage { // Regular old message to UserID
        to: UserID,
        from: UserID,
        msg: String
    },    
    Request { // Used for requesting data from DB and users.
        to: UserID,
        from: UserID,
        kind: RequestKind,
    },
    Response { // Server will modify user data (friends list)
        to: UserID,
        from: UserID,
        kind: ResponseKind, // Accepted or rejected
    },

    Register {
        from: UserID,
        #[serde(default)]
        psw: String,
    },
    LoginCredentials {
        from: UserID,
        #[serde(default)]
        psw: String,
    },

    Error(String),                  // Some sort of error?
    EOF, // stream terminated
}

/// C styled enum (actual numeric datatype) which will determine the kind
/// of data being requested.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestKind {
    UserInfo,
    ChatHistory,
    GroupHistory,

}

/// Response styled thingy yeah TODO: make a better explanation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseKind {
    UserInfo(HashMap<String, String>),
    // pulls messages from db into
    ChatHistory(HashMap<TimeStamp, Stanza>), 
    GroupHistory(HashMap<TimeStamp, Stanza>),

}

impl Stanza {

    /// Optionally returns the ID the message should be sent to.
    pub fn to(&self) -> Option<UserID> {
        match *self {
            Stanza::Message{ to, .. }       => Some(to.clone()),
            Stanza::GroupMessage{ to, .. }  => Some(to.clone()),
            Stanza::Request{ to, .. }       => Some(to.clone()),
            Stanza::Response{ to, .. }      => Some(to.clone()),
            Stanza::Error(_)                => None,
            _                               => panic!("Unimplemented")
        }
    }

    /// Optionally returns the ID the message should be sent to.
    pub fn from(&self) -> Option<UserID> {
        match *self {
            Stanza::Message{ from, .. }     => Some(from.clone()),
            Stanza::Request{ from, .. }     => Some(from.clone()),
            Stanza::Response{ from, .. }    => Some(from.clone()),
            Stanza::Error(_)                => None,
            _                               => panic!("Unimplemented")
        }
    }

    /// Processes a request, panics if given stanza is not in fact a request.
    #[allow(unused_variables)]
    pub fn process_request(self) -> Self {
        if let Stanza::Request{ to, from, kind } = self {
            unimplemented!()
        } else { panic!("Error, processesing non-request") }
    }
}

impl RequestKind {
    /// Single method implementation which creates a request item.
    pub fn respond(self, to: UserID, from: UserID) -> ResponseKind {
        unimplemented!()
        //match kind {
            //RequestKind::UserInfo => ,
            //RequestKind::ChatHistory => ,
            //RequestKind::GroupHistory => ,
        //}
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
                let object_buf = buf.drain_to(index + 1);
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

}

