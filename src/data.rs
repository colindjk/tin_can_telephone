// This file will include the Data (struct or enum) which will represent the
// different pieces of data which can be passed via HTTP style Request / Response
pub type UserID = SocketAddr; // right now we'll just id by port number for ease

use std::net::SocketAddr;
use tokio_core::io::{Io, Decode, Encode, EasyBuf};
use std::io::{Error};
//use tokio_core::io::frame::*;

#[deriving(sized)]
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

/// Decodability, this is where we decide on formatting.
impl Decode for Data {
    fn decode(buf: &mut EasyBuf) -> Result<Option<Self>, Error> {
        panic!("unimplemented");
    }
    fn done(buf: &mut EasyBuf) -> Result<Self, Error> {
        panic!("unimplemented");
    }
}

/// Encodability
impl Encode for Data {
    fn encode(self, buf: &mut Vec<u8>) {
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

