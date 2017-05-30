//! # reactions_get
//!
//! This method returns a list of all reactions for a single item (file, file comment, channel message, group message, or direct message).  
//! See https://api.slack.com/methods/reactions.get  
//!

//!
//! ## Optional arguments: 
//!
//!  `channel` (String)  
//!  Channel where the message to get reactions for was posted.  
//!  Example: C1234567890  
//!
//!  `file` (String)  
//!  File to get reactions for.  
//!  Example: F1234567890  
//!
//!  `file_comment` (String)  
//!  File comment to get reactions for.  
//!  Example: Fc1234567890  
//!
//!  `full` (String)  
//!  If true always return the complete reaction list.  
//!  Example:    
//!
//!  `timestamp` (f64)  
//!  Timestamp of the message to get reactions for.  
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
pub struct GetOptions {

/// Channel where the message to get reactions for was posted.
    pub channel: Option<String>,
/// File to get reactions for.
    pub file: Option<String>,
/// File comment to get reactions for.
    pub file_comment: Option<String>,
/// If true always return the complete reaction list.
    pub full: Option<String>,
/// Timestamp of the message to get reactions for.
    pub timestamp: Option<f64>,
}

impl<'a> GetOptions {

    /// Create a new instance of GetOptions 
    fn new<>() -> GetOptions {
        GetOptions { 
        
             ..Default::default()
        }
    }
}

impl From<GetOptions> for String {
    fn from(options: GetOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The GetBuilder provides a fluid interface to create
/// objects of type GetOptions
pub struct GetBuilder<'a> {
    client: &'a mut Client,
    params: GetOptions,
}

impl<'a> GetBuilder<'a> {

    /// Create a default GetBuilder object
    pub fn default<>(client: &'a mut Client, ) -> GetBuilder<'a> {
        GetBuilder {
            client: client,
            params: GetOptions::new(
            
            ),
        }
    }
/// Channel where the message to get reactions for was posted.
    pub fn channel<S: Into<String>>(&'a mut self, channel: S) -> &mut GetBuilder {
        self.params.channel = Some(channel.into());
        self
    }
/// File to get reactions for.
    pub fn file<S: Into<String>>(&'a mut self, file: S) -> &mut GetBuilder {
        self.params.file = Some(file.into());
        self
    }
/// File comment to get reactions for.
    pub fn file_comment<S: Into<String>>(&'a mut self, file_comment: S) -> &mut GetBuilder {
        self.params.file_comment = Some(file_comment.into());
        self
    }
/// If true always return the complete reaction list.
    pub fn full<S: Into<String>>(&'a mut self, full: S) -> &mut GetBuilder {
        self.params.full = Some(full.into());
        self
    }
/// Timestamp of the message to get reactions for.
    pub fn timestamp(&'a mut self, timestamp: f64) -> &mut GetBuilder {
        self.params.timestamp = Some(timestamp);
        self
    }

    /// Send the request to Slack and try to convert the response to a GetResponse
    pub fn send(&mut self) -> Result<ReactionsGetResponse> {
        let payload = self.client.send(format!("{}?{:?}", "reactions.get", &self.params))?;
        payload.to_type::<ReactionsGetResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn reactions_get<>(&mut self, ) -> GetBuilder {
        GetBuilder::default(self,
        
        )
    }
}