//! # reactions_add
//!
//! This method adds a reaction (emoji) to an item (file, file comment, channel message, group message, or direct message).  
//! One of file, file_comment, or the combination of channel and timestamp must be specified.  
//! See https://api.slack.com/methods/reactions.add  
//!
//! ## Required arguments: 
//!  `name` (String)  
//!  Reaction (emoji) name.  
//!  Example: thumbsup
//!
//! ## Optional arguments: 
//!
//!  `channel` (String)  
//!  Channel where the message to add reaction to was posted.  
//!  Example: C1234567890  
//!
//!  `file` (String)  
//!  File to add reaction to.  
//!  Example: F1234567890  
//!
//!  `file_comment` (String)  
//!  File comment to add reaction to.  
//!  Example: Fc1234567890  
//!
//!  `timestamp` (f64)  
//!  Timestamp of the message to add reaction to.  
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
pub struct AddOptions {
/// Reaction (emoji) name.
    pub name: String,
/// Channel where the message to add reaction to was posted.
    pub channel: Option<String>,
/// File to add reaction to.
    pub file: Option<String>,
/// File comment to add reaction to.
    pub file_comment: Option<String>,
/// Timestamp of the message to add reaction to.
    pub timestamp: Option<f64>,
}

impl<'a> AddOptions {

    /// Create a new instance of AddOptions 
    fn new<S: Into<String>>(name: S) -> AddOptions {
        AddOptions { 
        name: name.into(),
             ..Default::default()
        }
    }
}

impl From<AddOptions> for String {
    fn from(options: AddOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The AddBuilder provides a fluid interface to create
/// objects of type AddOptions
pub struct AddBuilder<'a> {
    client: &'a mut Client,
    params: AddOptions,
}

impl<'a> AddBuilder<'a> {

    /// Create a default AddBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, name: S) -> AddBuilder<'a> {
        AddBuilder {
            client: client,
            params: AddOptions::new(
            name,
            ),
        }
    }
/// Channel where the message to add reaction to was posted.
    pub fn channel<S: Into<String>>(&'a mut self, channel: S) -> &mut AddBuilder {
        self.params.channel = Some(channel.into());
        self
    }
/// File to add reaction to.
    pub fn file<S: Into<String>>(&'a mut self, file: S) -> &mut AddBuilder {
        self.params.file = Some(file.into());
        self
    }
/// File comment to add reaction to.
    pub fn file_comment<S: Into<String>>(&'a mut self, file_comment: S) -> &mut AddBuilder {
        self.params.file_comment = Some(file_comment.into());
        self
    }
/// Timestamp of the message to add reaction to.
    pub fn timestamp(&'a mut self, timestamp: f64) -> &mut AddBuilder {
        self.params.timestamp = Some(timestamp);
        self
    }

    /// Send the request to Slack and try to convert the response to a AddResponse
    pub fn send(&mut self) -> Result<ReactionsAddResponse> {
        let payload = self.client.send(format!("{}?{:?}", "reactions.add", &self.params))?;
        payload.to_type::<ReactionsAddResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn reactions_add<S: Into<String>>(&mut self, name: S) -> AddBuilder {
        AddBuilder::default(self,
        name.into(),
        )
    }
}