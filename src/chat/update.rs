//! # chat_update
//!
//! This method updates a message in a channel. Though related to chat.postMessage, some parameters of chat.update are handled differently.  
//! See https://api.slack.com/methods/chat.update  
//!
//! ## Required arguments: 
//!  `channel` (String)  
//!  Channel containing the message to be updated.  
//!  Example: C1234567890  
//!  `text` (String)  
//!  New text for the message, using the default formatting rules.  
//!  Example: Hello world  
//!  `ts` (f64)  
//!  Timestamp of the message to be updated.  
//!  Example: 1405894322.002768
//!
//! ## Optional arguments: 
//!
//!  `as_user` (bool)  
//!  Pass true to update the message as the authed user. Bot users in this context are considered authed users.  
//!  Example: true  
//!
//!  `attachments` (String)  
//!  Structured message attachments.  
//!  Example: [{"pretext": "pre-hello", "text": "text-world"}]  
//!
//!  `link_names` (bool)  
//!  Find and link channel names and usernames. Defaults to none. This parameter should be used in conjunction with parse. To set link_names to 1, specify a parse mode of full.  
//!  Example: 1  
//!
//!  `parse` (String)  
//!  Change how messages are treated. Defaults to client, unlike chat.postMessage. See below.  
//!  Example: none

// Warning! This file was auto-generated from the Slack API specification.
// Don't modify it directly. Modify the template that generates this file instead.

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate serde_urlencoded;

use client::Client;

use types::*;
use slack_types::*;

use errors::*;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateOptions {
/// Channel containing the message to be updated.
    pub channel: String,
/// New text for the message, using the default formatting rules.
    pub text: String,
/// Timestamp of the message to be updated.
    pub ts: f64,
/// Pass true to update the message as the authed user. Bot users in this context are considered authed users.
    pub as_user: Option<bool>,
/// Structured message attachments.
    pub attachments: Option<String>,
/// Find and link channel names and usernames. Defaults to none. This parameter should be used in conjunction with parse. To set link_names to 1, specify a parse mode of full.
    pub link_names: Option<bool>,
/// Change how messages are treated. Defaults to client, unlike chat.postMessage. See below.
    pub parse: Option<String>,
}

impl<'a> UpdateOptions {

    /// Create a new instance of UpdateOptions 
    fn new<S: Into<String>>(channel: S, text: S, ts: f64) -> UpdateOptions {
        UpdateOptions { 
        channel: channel.into(),
        text: text.into(),
        ts: ts,
             ..Default::default()
        }
    }
}

impl From<UpdateOptions> for String {
    fn from(options: UpdateOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The UpdateBuilder provides a fluid interface to create
/// objects of type UpdateOptions
pub struct UpdateBuilder<'a> {
    client: &'a mut Client,
    params: UpdateOptions,
}

impl<'a> UpdateBuilder<'a> {

    /// Create a default UpdateBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, channel: S, text: S, ts: f64) -> UpdateBuilder<'a> {
        UpdateBuilder {
            client: client,
            params: UpdateOptions::new(
            channel,
            text,
            ts,
            ),
        }
    }
/// Pass true to update the message as the authed user. Bot users in this context are considered authed users.
    pub fn as_user(&'a mut self, as_user: bool) -> &mut UpdateBuilder {
        self.params.as_user = Some(as_user);
        self
    }
/// Structured message attachments.
    pub fn attachments<S: Into<String>>(&'a mut self, attachments: S) -> &mut UpdateBuilder {
        self.params.attachments = Some(attachments.into());
        self
    }
/// Find and link channel names and usernames. Defaults to none. This parameter should be used in conjunction with parse. To set link_names to 1, specify a parse mode of full.
    pub fn link_names(&'a mut self, link_names: bool) -> &mut UpdateBuilder {
        self.params.link_names = Some(link_names);
        self
    }
/// Change how messages are treated. Defaults to client, unlike chat.postMessage. See below.
    pub fn parse<S: Into<String>>(&'a mut self, parse: S) -> &mut UpdateBuilder {
        self.params.parse = Some(parse.into());
        self
    }

    /// Send the request to Slack and try to convert the response to a UpdateResponse
    pub fn send(&mut self) -> Result<ChatUpdateResponse> {
        let payload = self.client.send(format!("{}?{:?}", "chat.update", &self.params))?;
        payload.to_type::<ChatUpdateResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn chat_update<S: Into<String>>(&mut self, channel: S, text: S, ts: f64) -> UpdateBuilder {
        UpdateBuilder::default(self,
        channel.into(),
        text.into(),
        ts,
        )
    }
}