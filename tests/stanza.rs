#![feature(proc_macro, plugin, custom_attribute, custom_derive, plugin)]
#![plugin(serde_derive)]

extern crate serde_json as json;
extern crate tin_can_telephone as tct;

use json::ser::{to_vec};
use json::de::{from_slice};

use tct::stanza::Stanza;

#[test]
fn stanza_message_format() {

    let message : Stanza = from_slice(
        "{\"Message\":{\"to\":\"colin\",\"from\":\"joe\",\"msg\":\"body example\"}}"
        .as_bytes()).unwrap();

    let matching_to = "colin".to_string();
    let matching_from = "joe".to_string();
    let matching_msg = "body example".to_string();

    if let Stanza::Message{ to, from, msg } = message {
        assert_eq!(to, matching_to);
        assert_eq!(from, matching_from);
        assert_eq!(msg, matching_msg);
    }

    let group_message : Stanza = from_slice(
        "{\"GroupMessage\":{\"to\":\"colin\",\"from\":\"joesteam\",\"msg\":\"body example\"}}"
        .as_bytes()).unwrap();

    let matching_to = "colin".to_string();
    let matching_from = "joesteam".to_string();
    let matching_msg = "body example".to_string();

    if let Stanza::GroupMessage{ to, from, msg, .. } = group_message {
        assert_eq!(to, matching_to);
        assert_eq!(from, matching_from);
        assert_eq!(msg, matching_msg);
    }

}

#[test]
/// This will test the format we use for requesting data from the server.
fn stanza_req_res_format() {

    use tct::stanza::{RequestKind, ResponseKind};
    use std::collections::{HashMap};

    let req : Stanza = from_slice(
        "{\"Request\":{\"to\":\"him\",\
        \"from\":\"her\",\"kind\":\"UserInfo\"}}"
        .as_bytes()).unwrap();

    let matching_to = "him".to_string();
    let matching_from = "her".to_string();
    let matching_kind = RequestKind::UserInfo; // Where UserInfo is a variant of an enum

    if let Stanza::Request{ to, from, kind } = req {
        assert_eq!(to, matching_to);
        assert_eq!(from, matching_from);
        assert_eq!(kind, matching_kind);
    }

    let res : Stanza = from_slice(
        "{\"Response\":{\"to\":\"guy\",\
        \"from\":\"girl\",\"kind\":{\"\
        UserInfo\":{\"one\":\"one_back\", \"two\":\"two_back\"}}}}"
        .as_bytes()).unwrap();

    let matching_to = "guy".to_string();
    let matching_from = "girl".to_string();
    let mut map : HashMap<String, String> = HashMap::new();
    map.insert("one".to_string(), "one_back".to_string());
    map.insert("two".to_string(), "two_back".to_string());

    if let Stanza::Response{ to, from, kind } = res {
        assert_eq!(to, matching_to);
        assert_eq!(from, matching_from);
        if let ResponseKind::UserInfo(res_map) = kind {
            assert_eq!(map.get("one"), res_map.get("one"));
            assert_eq!(map.get("two"), res_map.get("two"));
        }
    }

}

