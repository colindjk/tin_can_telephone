// This file will include the Data (struct or enum) which will represent the
// different pieces of data which can be passed via HTTP style Request / Response.

type UserID = u64;
//type UserMap = HashMap<UserID, Data::User>;

use tokio_core::io::*;
use std::io::{Error};
//use tokio_core::io::frame::*;

#[deriving(sized)]
pub enum Data {
    Message,
    Problem,
    Etc,
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

#[deriving(Debug, Serialize, Deserialize)]
pub struct Message {
    user_id: UserID,
    message_body: String,
    flags: u64,
}

#[deriving(Debug, Serialize, Deserialize)]
pub struct InfoRequest {

}

#[deriving(Debug, Serialize, Deserialize)]
pub struct User {
    user_id: UserID, // will be an username / email address.
}

enum Info {

}

