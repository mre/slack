//! # chat_delete
//!
//! This method deletes a message from a channel.  
//! See https://api.slack.com/methods/chat.delete  
//!
//! ## Required arguments: 
//!  `channel` (String)  
//!  Channel containing the message to be deleted.  
//!  Example: C1234567890  
//!  `ts` (f64)  
//!  Timestamp of the message to be deleted.  
//!  Example: 1405894322.002768
//!
//! ## Optional arguments: 
//!
//!  `as_user` (bool)  
//!  Pass true to delete the message as the authed user. Bot users in this context are considered authed users.  
//!  Example: true

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
pub struct DeleteOptions {
/// Channel containing the message to be deleted.
    pub channel: String,
/// Timestamp of the message to be deleted.
    pub ts: f64,
/// Pass true to delete the message as the authed user. Bot users in this context are considered authed users.
    pub as_user: Option<bool>,
}

impl<'a> DeleteOptions {

    /// Create a new instance of DeleteOptions 
    fn new<S: Into<String>>(channel: S, ts: f64) -> DeleteOptions {
        DeleteOptions { 
        channel: channel.into(),
        ts: ts,
             ..Default::default()
        }
    }
}

impl From<DeleteOptions> for String {
    fn from(options: DeleteOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The DeleteBuilder provides a fluid interface to create
/// objects of type DeleteOptions
pub struct DeleteBuilder<'a> {
    client: &'a mut Client,
    params: DeleteOptions,
}

impl<'a> DeleteBuilder<'a> {

    /// Create a default DeleteBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, channel: S, ts: f64) -> DeleteBuilder<'a> {
        DeleteBuilder {
            client: client,
            params: DeleteOptions::new(
            channel,
            ts,
            ),
        }
    }
/// Pass true to delete the message as the authed user. Bot users in this context are considered authed users.
    pub fn as_user(&'a mut self, as_user: bool) -> &mut DeleteBuilder {
        self.params.as_user = Some(as_user);
        self
    }

    /// Send the request to Slack and try to convert the response to a DeleteResponse
    pub fn send(&mut self) -> Result<ChatDeleteResponse> {
        let payload = self.client.send(format!("{}?{:?}", "chat.delete", &self.params))?;
        payload.to_type::<ChatDeleteResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn chat_delete<S: Into<String>>(&mut self, channel: S, ts: f64) -> DeleteBuilder {
        DeleteBuilder::default(self,
        channel.into(),
        ts,
        )
    }
}