// This file will include the Data (struct or enum) which will represent the
// different pieces of data which can be passed via HTTP style Request / Response.

type UserID = u64;
//type UserMap = HashMap<UserID, Data::User>;

pub enum Data {
    User {
        name: String
    },
    Group {
        id: u64
    },
    Message {
        text: String
    }
}
