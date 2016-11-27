#![feature(proc_macro, plugin, custom_attribute, custom_derive, plugin)]
#![plugin(serde_derive)]

// Imports:

// -- Parsing:
extern crate serde;
extern crate serde_json as json;
#[macro_use] 
extern crate serde_derive;

// -- Logs:
extern crate env_logger;

// -- Network:
extern crate futures;
extern crate tokio_core;

extern crate mysql_async as my;

// TODO: NEXT -> SERIALIZATION ULTIMATUM.
// TODO: mod server -> Client management, send / receive messages to clients
// TODO: mod client -> reads data given by a client, who writes back?
//       (format? -> next)
// TODO: mod xmpp -> formatting -> implement XmlStream struct.
// TODO: mod server, client? -> Implement logging ('log') for the XML stream.

// TCP or UDP?
// TCP.

// Note on the copy(reader, writer) function, it's very simple.
// The function takes data given to reader and copies it over to writer,
// writing back whatever was written. The 'copy' might as well be called 'echo'.

// Also, in this case, when an incoming socket happens, we cannot handle the split
// without a future, since the connection will persist far longer than the
// 'for_each' iteration will, therefore it must be run asynchronously, to avoid
// only being able to handle one client at a time.

pub mod server;
pub mod client;
pub mod stanza;
pub mod db;

