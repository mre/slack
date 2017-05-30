//! # stars_remove
//!
//! This method removes a star from an item (message, file, file comment, channel, private group, or DM) on behalf of the authenticated user.  
//! One of file, file_comment, channel, or the combination of channel and timestamp must be specified.  
//! See https://api.slack.com/methods/stars.remove  
//!

//!
//! ## Optional arguments: 
//!
//!  `channel` (String)  
//!  Channel to remove star from, or channel where the message to remove star from was posted (used with timestamp).  
//!  Example: C1234567890  
//!
//!  `file` (String)  
//!  File to remove star from.  
//!  Example: F1234567890  
//!
//!  `file_comment` (String)  
//!  File comment to remove star from.  
//!  Example: Fc1234567890  
//!
//!  `timestamp` (f64)  
//!  Timestamp of the message to remove star from.  
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

/// Channel to remove star from, or channel where the message to remove star from was posted (used with timestamp).
    pub channel: Option<String>,
/// File to remove star from.
    pub file: Option<String>,
/// File comment to remove star from.
    pub file_comment: Option<String>,
/// Timestamp of the message to remove star from.
    pub timestamp: Option<f64>,
}

impl<'a> RemoveOptions {

    /// Create a new instance of RemoveOptions 
    fn new<>() -> RemoveOptions {
        RemoveOptions { 
        
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
    pub fn default<>(client: &'a mut Client, ) -> RemoveBuilder<'a> {
        RemoveBuilder {
            client: client,
            params: RemoveOptions::new(
            
            ),
        }
    }
/// Channel to remove star from, or channel where the message to remove star from was posted (used with timestamp).
    pub fn channel<S: Into<String>>(&'a mut self, channel: S) -> &mut RemoveBuilder {
        self.params.channel = Some(channel.into());
        self
    }
/// File to remove star from.
    pub fn file<S: Into<String>>(&'a mut self, file: S) -> &mut RemoveBuilder {
        self.params.file = Some(file.into());
        self
    }
/// File comment to remove star from.
    pub fn file_comment<S: Into<String>>(&'a mut self, file_comment: S) -> &mut RemoveBuilder {
        self.params.file_comment = Some(file_comment.into());
        self
    }
/// Timestamp of the message to remove star from.
    pub fn timestamp(&'a mut self, timestamp: f64) -> &mut RemoveBuilder {
        self.params.timestamp = Some(timestamp);
        self
    }

    /// Send the request to Slack and try to convert the response to a RemoveResponse
    pub fn send(&mut self) -> Result<StarsRemoveResponse> {
        let payload = self.client.send(format!("{}?{:?}", "stars.remove", &self.params))?;
        payload.to_type::<StarsRemoveResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn stars_remove<>(&mut self, ) -> RemoveBuilder {
        RemoveBuilder::default(self,
        
        )
    }
}