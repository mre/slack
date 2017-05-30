//! # groups_open
//!
//! This method opens a private channel.  
//! See https://api.slack.com/methods/groups.open  
//!
//! ## Required arguments: 
//!  `channel` (String)  
//!  Private channel to open.  
//!  Example: G1234567890


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
pub struct OpenOptions {
/// Private channel to open.
    pub channel: String,

}

impl<'a> OpenOptions {

    /// Create a new instance of OpenOptions 
    fn new<S: Into<String>>(channel: S) -> OpenOptions {
        OpenOptions { 
        channel: channel.into(),
             ..Default::default()
        }
    }
}

impl From<OpenOptions> for String {
    fn from(options: OpenOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The OpenBuilder provides a fluid interface to create
/// objects of type OpenOptions
pub struct OpenBuilder<'a> {
    client: &'a mut Client,
    params: OpenOptions,
}

impl<'a> OpenBuilder<'a> {

    /// Create a default OpenBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, channel: S) -> OpenBuilder<'a> {
        OpenBuilder {
            client: client,
            params: OpenOptions::new(
            channel,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a OpenResponse
    pub fn send(&mut self) -> Result<GroupsOpenResponse> {
        let payload = self.client.send(format!("{}?{:?}", "groups.open", &self.params))?;
        payload.to_type::<GroupsOpenResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn groups_open<S: Into<String>>(&mut self, channel: S) -> OpenBuilder {
        OpenBuilder::default(self,
        channel.into(),
        )
    }
}