//! # channels_join
//!
//! This method is used to join a channel. If the channel does not exist, it is  
//! created.  
//! See https://api.slack.com/methods/channels.join  
//!
//! ## Required arguments: 
//!  `name` (String)  
//!  Name of channel to join.  
//!  Example:


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
pub struct JoinOptions {
/// Name of channel to join.
    pub name: String,

}

impl<'a> JoinOptions {

    /// Create a new instance of JoinOptions 
    fn new<S: Into<String>>(name: S) -> JoinOptions {
        JoinOptions { 
        name: name.into(),
             ..Default::default()
        }
    }
}

impl From<JoinOptions> for String {
    fn from(options: JoinOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The JoinBuilder provides a fluid interface to create
/// objects of type JoinOptions
pub struct JoinBuilder<'a> {
    client: &'a mut Client,
    params: JoinOptions,
}

impl<'a> JoinBuilder<'a> {

    /// Create a default JoinBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, name: S) -> JoinBuilder<'a> {
        JoinBuilder {
            client: client,
            params: JoinOptions::new(
            name,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a JoinResponse
    pub fn send(&mut self) -> Result<ChannelsJoinResponse> {
        let payload = self.client.send(format!("{}?{:?}", "channels.join", &self.params))?;
        payload.to_type::<ChannelsJoinResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn channels_join<S: Into<String>>(&mut self, name: S) -> JoinBuilder {
        JoinBuilder::default(self,
        name.into(),
        )
    }
}