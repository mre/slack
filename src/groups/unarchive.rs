//! # groups_unarchive
//!
//! This method unarchives a private channel.  
//! See https://api.slack.com/methods/groups.unarchive  
//!
//! ## Required arguments: 
//!  `channel` (String)  
//!  Private channel to unarchive.  
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
pub struct UnarchiveOptions {
/// Private channel to unarchive.
    pub channel: String,

}

impl<'a> UnarchiveOptions {

    /// Create a new instance of UnarchiveOptions 
    fn new<S: Into<String>>(channel: S) -> UnarchiveOptions {
        UnarchiveOptions { 
        channel: channel.into(),
             ..Default::default()
        }
    }
}

impl From<UnarchiveOptions> for String {
    fn from(options: UnarchiveOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The UnarchiveBuilder provides a fluid interface to create
/// objects of type UnarchiveOptions
pub struct UnarchiveBuilder<'a> {
    client: &'a mut Client,
    params: UnarchiveOptions,
}

impl<'a> UnarchiveBuilder<'a> {

    /// Create a default UnarchiveBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, channel: S) -> UnarchiveBuilder<'a> {
        UnarchiveBuilder {
            client: client,
            params: UnarchiveOptions::new(
            channel,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a UnarchiveResponse
    pub fn send(&mut self) -> Result<GroupsUnarchiveResponse> {
        let payload = self.client.send(format!("{}?{:?}", "groups.unarchive", &self.params))?;
        payload.to_type::<GroupsUnarchiveResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn groups_unarchive<S: Into<String>>(&mut self, channel: S) -> UnarchiveBuilder {
        UnarchiveBuilder::default(self,
        channel.into(),
        )
    }
}