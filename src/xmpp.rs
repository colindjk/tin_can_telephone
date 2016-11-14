// TODO: Implement reading XmlStream struct: An absraction over a TCP stream
//       sending data back and forth, handling all setup beyond binding the
//       tokio_core::net::TcpStream struct.

use std::collections::HashMap; 

use std::io::{Read, Write};

use futures::{Future, Poll};
use futures::stream::Stream;

use serde_xml::from_str;
use serde_xml::value::{Element, from_value};
use serde_xml::Error;

/// This will contain a hashmap for the possible XML messages / formats.
/// Type 'D' for the Data type which will be returned by the function
/// passed in via HashMap.
struct XmlStream<S> where S : Read + Write {
    stream: S,
    stanzas: Vec<String>, // Describes the various types of messages.
}

// Note:
// XmlStreams will be unaware of domains, as domains are a characteristic of
// network streams, an XmlStream should be looked at as a (asynch?) parser.

impl<S> XmlStream<S> where S : Read + Write {
    /// Consumes a stream for the purpose of reading a 'stream' of XML elements
    /// sent from a client.
    fn new(stream : S) -> XmlStream<S> {
        XmlStream {
            stream: stream,
            stanzas: vec![],
        }
    }
}

impl<S> Stream for XmlStream<S> where S : Read + Write {
    type Item = String;
    type Error = String;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        Err("Nope".to_string()) // uhhh fix this l8er
    }
}

// -- Old code:
// Helpful typedefs for the values read in and out of the list.
//type Ser<D> = Box<Fn(D)       -> Option<Vec<u8>>>;
//type De<D>  = Box<Fn(Vec<u8>) -> Option<D>>;

