//! # groups_info
//!
//! This method returns information about a private channel.  
//! See https://api.slack.com/methods/groups.info  
//!
//! ## Required arguments: 
//!  `channel` (String)  
//!  Private channel to get info on.  
//!  Example: C1234567890


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
pub struct InfoOptions {
/// Private channel to get info on.
    pub channel: String,

}

impl<'a> InfoOptions {

    /// Create a new instance of InfoOptions 
    fn new<S: Into<String>>(channel: S) -> InfoOptions {
        InfoOptions { 
        channel: channel.into(),
             ..Default::default()
        }
    }
}

impl From<InfoOptions> for String {
    fn from(options: InfoOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The InfoBuilder provides a fluid interface to create
/// objects of type InfoOptions
pub struct InfoBuilder<'a> {
    client: &'a mut Client,
    params: InfoOptions,
}

impl<'a> InfoBuilder<'a> {

    /// Create a default InfoBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, channel: S) -> InfoBuilder<'a> {
        InfoBuilder {
            client: client,
            params: InfoOptions::new(
            channel,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a InfoResponse
    pub fn send(&mut self) -> Result<GroupsInfoResponse> {
        let payload = self.client.send(format!("{}?{:?}", "groups.info", &self.params))?;
        payload.to_type::<GroupsInfoResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn groups_info<S: Into<String>>(&mut self, channel: S) -> InfoBuilder {
        InfoBuilder::default(self,
        channel.into(),
        )
    }
}