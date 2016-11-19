// This file will include the Data (struct or enum) which will represent the
// different pieces of data which can be passed via HTTP style Request / Response
pub type UserID = SocketAddr; // right now we'll just id by port number for ease

use std::net::SocketAddr;
use tokio_core::io::{Io, Codec, EasyBuf};
use std::io::{Error};
//use tokio_core::io::frame::*;

pub type Message = Result<Data, String>;

#[deriving(sized)]
#[derive(Clone)]
/// The IMMUTABLE struct which is passed between threads etc in order
/// to send and receive messages.
pub enum Data {
    Message(UserID, String),    // Regular old message to UserID
    Request(UserID),            // friend request to user
    GroupRequest(UserID),       // Group invite request
    Error(String),              // Some sort of error?
}

impl Data {
    /// Optionally returns the ID the message should be sent to.
    pub fn id(&self) -> Option<UserID> {
        match *self {
            Data::Message(id, _)    => Some(id.clone()),
            Data::Request(id)       => Some(id.clone()),
            Data::GroupRequest(id)  => Some(id.clone()),
            Data::Error(_)          => None,
        }
    }
}

pub struct DataParser;

/// Decodability, this is where we decide on formatting.
impl Codec for DataParser {
    type In = Data;
    type Out = Data;

    /// Reads the EasyBuf.
    fn decode(&mut self, buf: &mut EasyBuf) -> Result<Option<Self::In>, Error> {
        panic!("unimplemented");
    }

    /// Fills the buffer with the consumed 'Out' message.
    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) {
        panic!("unimplemented");
    }

    /// Reads until the end of stream, 
    fn decode_eof(&mut self, buf: &mut EasyBuf) -> Result<Self::In, Error> {
        panic!("unimplemented");
    }
}

//#[deriving(Debug, Serialize, Deserialize)]
//pub struct Message {
    //user_id: UserID,
    //message_body: String,
    //flags: u64,
//}

//#[deriving(Debug, Serialize, Deserialize)]
//pub struct InfoRequest {

//}

//#[deriving(Debug, Serialize, Deserialize)]
//pub struct User {
    //user_id: UserID, // will be an username / email address.
//}

//enum Info {

//}

