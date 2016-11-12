//#![feature(custom_derive, plugin, test)]
//#![feature(custom_attribute)]
//#![plugin(serde_macros)]

//extern crate serde;
//extern crate serde_xml;

use std::collections::HashMap; 

use futures::{Future, Poll};
use futures::stream::Stream;

//use tokio_core::io::{copy, Io};
use tokio_core::net::{TcpStream};
//use tokio_core::reactor::Core;

use data::Data;

struct XmlParser {

}

// Helpful typedefs for the values read in and out of the list.
type Ser<D> = Box<Fn(D)       -> Option<Vec<u8>>>;
type De<D>  = Box<Fn(Vec<u8>) -> Option<D>>;

/// This will contain a hashmap for the possible XML messages / formats.
/// Type 'D' for the Data type which will be returned by the function
/// passed in via HashMap.
struct XmlStream<D> {
    stream: TcpStream,
    id: String,
    host: String,
    ser: HashMap<String, Ser<D>>, // probably will want to use slices here instead.
    de: HashMap<String, De<D>>, 
}

impl<D> XmlStream<D> {

}

impl<D> Stream for XmlStream<D> {
    type Item = Data;
    type Error = Result<(), ()>;
    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        Err(Err(())) // uhhh fix this l8er
    }
}

