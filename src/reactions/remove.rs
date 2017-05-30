//! # reactions_remove
//!
//! This method removes a reaction (emoji) from an item (file, file comment, channel message, group message, or direct message).  
//! One of file, file_comment, or the combination of channel and timestamp must be specified.  
//! See https://api.slack.com/methods/reactions.remove  
//!
//! ## Required arguments: 
//!  `name` (String)  
//!  Reaction (emoji) name.  
//!  Example: thumbsup
//!
//! ## Optional arguments: 
//!
//!  `channel` (String)  
//!  Channel where the message to remove reaction from was posted.  
//!  Example: C1234567890  
//!
//!  `file` (String)  
//!  File to remove reaction from.  
//!  Example: F1234567890  
//!
//!  `file_comment` (String)  
//!  File comment to remove reaction from.  
//!  Example: Fc1234567890  
//!
//!  `timestamp` (f64)  
//!  Timestamp of the message to remove reaction from.  
//!  Example: 1234567890.123456

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
pub struct RemoveOptions {
/// Reaction (emoji) name.
    pub name: String,
/// Channel where the message to remove reaction from was posted.
    pub channel: Option<String>,
/// File to remove reaction from.
    pub file: Option<String>,
/// File comment to remove reaction from.
    pub file_comment: Option<String>,
/// Timestamp of the message to remove reaction from.
    pub timestamp: Option<f64>,
}

impl<'a> RemoveOptions {

    /// Create a new instance of RemoveOptions 
    fn new<S: Into<String>>(name: S) -> RemoveOptions {
        RemoveOptions { 
        name: name.into(),
             ..Default::default()
        }
    }
}

impl From<RemoveOptions> for String {
    fn from(options: RemoveOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The RemoveBuilder provides a fluid interface to create
/// objects of type RemoveOptions
pub struct RemoveBuilder<'a> {
    client: &'a mut Client,
    params: RemoveOptions,
}

impl<'a> RemoveBuilder<'a> {

    /// Create a default RemoveBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, name: S) -> RemoveBuilder<'a> {
        RemoveBuilder {
            client: client,
            params: RemoveOptions::new(
            name,
            ),
        }
    }
/// Channel where the message to remove reaction from was posted.
    pub fn channel<S: Into<String>>(&'a mut self, channel: S) -> &mut RemoveBuilder {
        self.params.channel = Some(channel.into());
        self
    }
/// File to remove reaction from.
    pub fn file<S: Into<String>>(&'a mut self, file: S) -> &mut RemoveBuilder {
        self.params.file = Some(file.into());
        self
    }
/// File comment to remove reaction from.
    pub fn file_comment<S: Into<String>>(&'a mut self, file_comment: S) -> &mut RemoveBuilder {
        self.params.file_comment = Some(file_comment.into());
        self
    }
/// Timestamp of the message to remove reaction from.
    pub fn timestamp(&'a mut self, timestamp: f64) -> &mut RemoveBuilder {
        self.params.timestamp = Some(timestamp);
        self
    }

    /// Send the request to Slack and try to convert the response to a RemoveResponse
    pub fn send(&mut self) -> Result<ReactionsRemoveResponse> {
        let payload = self.client.send(format!("{}?{:?}", "reactions.remove", &self.params))?;
        payload.to_type::<ReactionsRemoveResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn reactions_remove<S: Into<String>>(&mut self, name: S) -> RemoveBuilder {
        RemoveBuilder::default(self,
        name.into(),
        )
    }
}