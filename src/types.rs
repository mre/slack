extern crate serde;
extern crate serde_json;
extern crate serde_urlencoded;
use std;
use errors::*;

use self::serde::{Deserialize, Deserializer};
use std::io::Read;
use hyper::client::Response as HyperResponse;

#[derive(Debug)]
pub struct Response {
    pub hyper_response: HyperResponse,
    pub body: String,
}

impl Response {
    pub fn from_hyper_response(mut hyper_response: HyperResponse) -> Result<Response> {
        let mut body = String::new();
        let _ = hyper_response.read_to_string(&mut body)
            .chain_err(|| "Unable to read Slack response");

        Ok(Response {
            hyper_response: hyper_response,
            body: body,
        })
    }

    /// Deserializes the body of the response from JSON into
    /// a `T`.
    pub fn to_type<T: Deserialize>(&self) -> Result<T> {
        serde_json::from_str(&*self.body)
            .chain_err(|| "Unable to convert response body to a Slack type")
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Object {
    value: String,
    creator: String,
    last_set: u64,
}

pub type Username = String;
type Topic = Object;
type Purpose = Object;

/// A channel object contains information about a team channel.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    id: String,
    name: String,
    is_channel: bool,
    created: u64,
    creator: Username,
    is_archived: bool,
    is_general: bool,
    members: Vec<String>,
    topic: Topic,
    purpose: Purpose,
    is_member: bool,
    last_read: f64,
    // latest: { ... }
    unread_count: u64,
    unread_count_display: u64,
}

/// A slack reaction (i.e. an emoji)
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Reaction {
    name: String,
    count: u64,
    users: Vec<Username>,
}

// #[derive(Debug, PartialEq, Serialize, Deserialize)]
// pub struct ImHistoryResponse {
//     pub ok: bool,
//     pub latest: Option<f64>,
//     pub messages: Vec<Message>,
//     pub has_more: bool,
// }

// Default types when fields are missing at deserialization.
fn vec_emtpy<T>() -> Vec<T> {
    vec![]
}

fn bool_false() -> bool {
    false
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Eq)]
pub enum MessageType {
    Message,
    Im,
    Group,
    File,
    FileComment,
    Channel,
    Unknown,
}

fn deserialize_message_type<D>(deserializer: D) -> std::result::Result<MessageType, D::Error>
    where D: Deserializer
{
    // Can't chain the serde::Deserializer::Error here because it is not `Size`d
    // Mapping to own error type instead. There must be a better way, though.
    let deser_result: serde_json::Value = serde::Deserialize::deserialize(deserializer)?;
    match deser_result {
        serde_json::Value::String(ref s) if &*s == "message" => Ok(MessageType::Message),
        serde_json::Value::String(ref s) if &*s == "im" => Ok(MessageType::Im),
        _ => Ok(MessageType::Unknown),
    }
}

/// A slack message
/// NOTE: Slack's `type` field was renamed to `msg_type`
/// because `type` is a reserved keyword in Rust.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "type", deserialize_with="deserialize_message_type")]
    pub msg_type: MessageType,
    pub ts: f64,
    pub user: Option<String>,
    pub text: Option<String>,
    #[serde(default = "bool_false")]
    pub is_starred: bool,
    #[serde(default = "bool_false")]
    pub wibblr: bool, // wtf is this!?
    #[serde(default = "vec_emtpy")]
    pub reactions: Vec<Reaction>,
}

#[test]
fn decode_slack_error() {
    let raw_json = r#"{
        "ok": false,
        "error": "channel_not_found"
    }"#;

    let error: SlackErrorMessage = serde_json::from_str(&raw_json).unwrap();
    assert!(error.error == SlackErrorType::ChannelNotFound);
}

#[test]
fn decode_channel_type() {
    let raw_json = r#"{
        "id": "C024BE91L",
        "name": "fun",
        "is_channel": "true",
        "created": 1360782804,
        "creator": "U024BE7LH",
        "is_archived": false,
        "is_general": false,
        "members": [
            "U024BE7LH",
            "U034BE7LH"
        ],
        "topic": {
            "value": "Fun times",
            "creator": "U024BE7LV",
            "last_set": 1369677212
        },
        "purpose": {
            "value": "This channel is for fun",
            "creator": "U024BE7LH",
            "last_set": 1360782804
        },
        "is_member": "true",
        "last_read": "1401383885.000061",
        "unread_count": 0,
        "unread_count_display": 0
    }"#;

    // Missing field: "latest": { ... }
    let channel: Channel = serde_json::from_str(&raw_json).unwrap();
    assert!(channel.created == 1360782804);
    assert!(channel.is_general == false);
}

#[test]
fn decode_reaction() {
    let raw_json = r#"{
        "name": "sweet_potato",
        "count": 5,
        "users": [ "U1", "U2", "U3", "U4", "U5" ]
    }"#;

    let reaction: Reaction = serde_json::from_str(&raw_json).unwrap();
    assert!(reaction.name == "sweet_potato");
    assert!(reaction.count == 5);
    assert!(reaction.users.len() == 5);
}

#[test]
fn decode_message_simple() {
    let raw_json = r#"{
        "type": "message",
        "ts": "1358546515.000008",
        "user": "U2147483896",
        "text": "Hello"
    }"#;
    let message: Message = serde_json::from_str(&raw_json).unwrap();
    assert!(message.msg_type == MessageType::Message); // TODO: this should be a proper Rustlang type
    assert!(message.ts == 1358546515.000008);
    assert!(message.user == Some("U2147483896".to_string()));
    assert!(message.text == Some("Hello".to_string()));
    assert!(message.is_starred == false);
    assert!(message.reactions.is_empty());
}

#[test]
fn decode_message_complex() {
    let raw_json = r#"{
        "type": "message",
        "ts": "1358546515.000007",
        "user": "matthias",
        "text": "World",
        "is_starred": "true",
        "wibblr": "true",
        "reactions": [
            {
                "name": "space_invader",
                "count": 3,
                "users": [ "U1", "U2", "U3" ]
            },
            {
                "name": "sweet_potato",
                "count": 2,
                "users": [ "U1", "U2" ]
            }
        ]
    }"#;
    let message: Message = serde_json::from_str(&raw_json).unwrap();
    assert!(message.msg_type == MessageType::Message); // TODO: this should be a proper Rustlang type
    assert!(message.ts == 1358546515.000007);
    assert!(message.user == Some("matthias".to_string()));
    assert!(message.text == Some("World".to_string()));
    assert!(message.is_starred == true);
    assert!(message.wibblr == true);
    assert!(message.reactions.len() == 2);
    assert!(message.reactions == vec![
        Reaction{name: "space_invader".to_string(), count: 3, users: vec!["U1".to_string(), "U2".to_string(), "U3".to_string()]},
        Reaction{name: "sweet_potato".to_string(), count: 2, users: vec!["U1".to_string(), "U2".to_string()]},
    ]);
}

#[test]
fn decode_im_history_minimal() {
    let raw_json = r#"{
        "ok": true,
        "messages": [],
        "has_more": false
    }"#;

    let im_history: ImHistoryResponse = serde_json::from_str(&raw_json).unwrap();
    assert!(im_history.ok == true);
    assert!(im_history.messages.len() == 0);
    assert!(im_history.has_more == false);
}


#[test]
fn decode_im_history_with_latest() {
    let raw_json = r#"{
        "ok": true,
        "latest": "1358547726.000003",
        "messages": [
            {
                "type": "message",
                "ts": "1358546515.000008",
                "user": "U2147483896",
                "text": "Hello"
            },
            {
                "type": "message",
                "ts": "1358546515.000007",
                "user": "U2147483896",
                "text": "World",
                "is_starred": true
            },
            {
                "type": "im",
                "ts": "1358546515.000007",
                "wibblr": true
            }
        ],
        "has_more": false
    }"#;

    let im_history: ImHistoryResponse = serde_json::from_str(&raw_json).unwrap();
    assert!(im_history.ok == true);
    assert!(im_history.latest == Some(1358547726.000003));
    assert!(im_history.messages.len() == 3);
    assert!(im_history.messages ==
            vec![Message {
                     msg_type: MessageType::Message,
                     ts: 1358546515.000008,
                     user: Some("U2147483896".to_string()),
                     text: Some("Hello".to_string()),
                     is_starred: false,
                     wibblr: false,
                     reactions: vec![],
                 },
                 Message {
                     msg_type: MessageType::Message,
                     ts: 1358546515.000007,
                     user: Some("U2147483896".to_string()),
                     text: Some("World".to_string()),
                     is_starred: true,
                     wibblr: false,
                     reactions: vec![],
                 },
                 Message {
                     msg_type: MessageType::Im,
                     ts: 1358546515.000007,
                     user: None,
                     text: None,
                     is_starred: false,
                     wibblr: true,
                     reactions: vec![],
                 }]);
    assert!(im_history.has_more == false);
}

#[test]
fn decode_im_history_without_latest() {
    let raw_json = r#"{
        "has_more" : false,
        "ok" : "true",
        "messages" : [
            {
                "type" : "message",
                "user" : "12345",
                "text" : "awesome",
                "ts" : "1470860481.000003"
            },
            {
                "user" : "12345",
                "type" : "message",
                "ts" : "1470860464.000002",
                "text" : "perfect"
            }
        ]
        }"#;

    let im_history: ImHistoryResponse = serde_json::from_str(&raw_json).unwrap();
    assert!(im_history.ok == true);
    assert!(im_history.latest == None);
    assert!(im_history.messages.len() == 2);
    assert!(im_history.messages ==
            vec![Message {
                     msg_type: MessageType::Message,
                     ts: 1470860481.000003,
                     user: Some("12345".to_string()),
                     text: Some("awesome".to_string()),
                     is_starred: false,
                     wibblr: false,
                     reactions: vec![],
                 },
                 Message {
                     msg_type: MessageType::Message,
                     ts: 1470860464.000002,
                     user: Some("12345".to_string()),
                     text: Some("perfect".to_string()),
                     is_starred: false,
                     wibblr: false,
                     reactions: vec![],
                 }]);
    assert!(im_history.has_more == false);
}
