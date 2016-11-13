// This file will include the Data (struct or enum) which will represent the
// different pieces of data which can be passed via HTTP style Request / Response.

type UserID = u64;
//type UserMap = HashMap<UserID, Data::User>;

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

