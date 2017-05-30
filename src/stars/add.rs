//! # stars_add
//!
//! This method adds a star to an item (message, file, file comment, channel, private group, or DM) on behalf of the authenticated user.  
//! One of file, file_comment, channel, or the combination of channel and timestamp must be specified.  
//! See https://api.slack.com/methods/stars.add  
//!

//!
//! ## Optional arguments: 
//!
//!  `channel` (String)  
//!  Channel to add star to, or channel where the message to add star to was posted (used with timestamp).  
//!  Example: C1234567890  
//!
//!  `file` (String)  
//!  File to add star to.  
//!  Example: F1234567890  
//!
//!  `file_comment` (String)  
//!  File comment to add star to.  
//!  Example: Fc1234567890  
//!
//!  `timestamp` (f64)  
//!  Timestamp of the message to add star to.  
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

/// Channel to add star to, or channel where the message to add star to was posted (used with timestamp).
    pub channel: Option<String>,
/// File to add star to.
    pub file: Option<String>,
/// File comment to add star to.
    pub file_comment: Option<String>,
/// Timestamp of the message to add star to.
    pub timestamp: Option<f64>,
}

impl<'a> AddOptions {

    /// Create a new instance of AddOptions 
    fn new<>() -> AddOptions {
        AddOptions { 
        
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
    pub fn default<>(client: &'a mut Client, ) -> AddBuilder<'a> {
        AddBuilder {
            client: client,
            params: AddOptions::new(
            
            ),
        }
    }
/// Channel to add star to, or channel where the message to add star to was posted (used with timestamp).
    pub fn channel<S: Into<String>>(&'a mut self, channel: S) -> &mut AddBuilder {
        self.params.channel = Some(channel.into());
        self
    }
/// File to add star to.
    pub fn file<S: Into<String>>(&'a mut self, file: S) -> &mut AddBuilder {
        self.params.file = Some(file.into());
        self
    }
/// File comment to add star to.
    pub fn file_comment<S: Into<String>>(&'a mut self, file_comment: S) -> &mut AddBuilder {
        self.params.file_comment = Some(file_comment.into());
        self
    }
/// Timestamp of the message to add star to.
    pub fn timestamp(&'a mut self, timestamp: f64) -> &mut AddBuilder {
        self.params.timestamp = Some(timestamp);
        self
    }

    /// Send the request to Slack and try to convert the response to a AddResponse
    pub fn send(&mut self) -> Result<StarsAddResponse> {
        let payload = self.client.send(format!("{}?{:?}", "stars.add", &self.params))?;
        payload.to_type::<StarsAddResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn stars_add<>(&mut self, ) -> AddBuilder {
        AddBuilder::default(self,
        
        )
    }
}