//! # pins_remove
//!
//! This method un-pins an item (file, file comment, channel message, or group message) from a channel.  
//! The channel argument is required and one of file, file_comment, or timestamp must also be specified.  
//! See https://api.slack.com/methods/pins.remove  
//!
//! ## Required arguments: 
//!  `channel` (String)  
//!  Channel where the item is pinned to.  
//!  Example: C1234567890
//!
//! ## Optional arguments: 
//!
//!  `file` (String)  
//!  File to un-pin.  
//!  Example: F1234567890  
//!
//!  `file_comment` (String)  
//!  File comment to un-pin.  
//!  Example: Fc1234567890  
//!
//!  `timestamp` (f64)  
//!  Timestamp of the message to un-pin.  
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
/// Channel where the item is pinned to.
    pub channel: String,
/// File to un-pin.
    pub file: Option<String>,
/// File comment to un-pin.
    pub file_comment: Option<String>,
/// Timestamp of the message to un-pin.
    pub timestamp: Option<f64>,
}

impl<'a> RemoveOptions {

    /// Create a new instance of RemoveOptions 
    fn new<S: Into<String>>(channel: S) -> RemoveOptions {
        RemoveOptions { 
        channel: channel.into(),
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
    pub fn default<S: Into<String>>(client: &'a mut Client, channel: S) -> RemoveBuilder<'a> {
        RemoveBuilder {
            client: client,
            params: RemoveOptions::new(
            channel,
            ),
        }
    }
/// File to un-pin.
    pub fn file<S: Into<String>>(&'a mut self, file: S) -> &mut RemoveBuilder {
        self.params.file = Some(file.into());
        self
    }
/// File comment to un-pin.
    pub fn file_comment<S: Into<String>>(&'a mut self, file_comment: S) -> &mut RemoveBuilder {
        self.params.file_comment = Some(file_comment.into());
        self
    }
/// Timestamp of the message to un-pin.
    pub fn timestamp(&'a mut self, timestamp: f64) -> &mut RemoveBuilder {
        self.params.timestamp = Some(timestamp);
        self
    }

    /// Send the request to Slack and try to convert the response to a RemoveResponse
    pub fn send(&mut self) -> Result<PinsRemoveResponse> {
        let payload = self.client.send(format!("{}?{:?}", "pins.remove", &self.params))?;
        payload.to_type::<PinsRemoveResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn pins_remove<S: Into<String>>(&mut self, channel: S) -> RemoveBuilder {
        RemoveBuilder::default(self,
        channel.into(),
        )
    }
}