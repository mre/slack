//! # pins_add
//!
//! This method pins an item (file, file comment, channel message, or group message) to a particular channel.  
//! The channel argument is required and one of file, file_comment, or timestamp must also be specified.  
//! See https://api.slack.com/methods/pins.add  
//!
//! ## Required arguments: 
//!  `channel` (String)  
//!  Channel to pin the item in.  
//!  Example: C1234567890
//!
//! ## Optional arguments: 
//!
//!  `file` (String)  
//!  File to pin.  
//!  Example: F1234567890  
//!
//!  `file_comment` (String)  
//!  File comment to pin.  
//!  Example: Fc1234567890  
//!
//!  `timestamp` (f64)  
//!  Timestamp of the message to pin.  
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
/// Channel to pin the item in.
    pub channel: String,
/// File to pin.
    pub file: Option<String>,
/// File comment to pin.
    pub file_comment: Option<String>,
/// Timestamp of the message to pin.
    pub timestamp: Option<f64>,
}

impl<'a> AddOptions {

    /// Create a new instance of AddOptions 
    fn new<S: Into<String>>(channel: S) -> AddOptions {
        AddOptions { 
        channel: channel.into(),
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
    pub fn default<S: Into<String>>(client: &'a mut Client, channel: S) -> AddBuilder<'a> {
        AddBuilder {
            client: client,
            params: AddOptions::new(
            channel,
            ),
        }
    }
/// File to pin.
    pub fn file<S: Into<String>>(&'a mut self, file: S) -> &mut AddBuilder {
        self.params.file = Some(file.into());
        self
    }
/// File comment to pin.
    pub fn file_comment<S: Into<String>>(&'a mut self, file_comment: S) -> &mut AddBuilder {
        self.params.file_comment = Some(file_comment.into());
        self
    }
/// Timestamp of the message to pin.
    pub fn timestamp(&'a mut self, timestamp: f64) -> &mut AddBuilder {
        self.params.timestamp = Some(timestamp);
        self
    }

    /// Send the request to Slack and try to convert the response to a AddResponse
    pub fn send(&mut self) -> Result<PinsAddResponse> {
        let payload = self.client.send(format!("{}?{:?}", "pins.add", &self.params))?;
        payload.to_type::<PinsAddResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn pins_add<S: Into<String>>(&mut self, channel: S) -> AddBuilder {
        AddBuilder::default(self,
        channel.into(),
        )
    }
}