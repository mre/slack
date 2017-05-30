//! # mpim_close
//!
//! This method closes a multiparty direct message channel.  
//! See https://api.slack.com/methods/mpim.close  
//!
//! ## Required arguments: 
//!  `channel` (String)  
//!  MPIM to close.  
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
pub struct CloseOptions {
/// MPIM to close.
    pub channel: String,

}

impl<'a> CloseOptions {

    /// Create a new instance of CloseOptions 
    fn new<S: Into<String>>(channel: S) -> CloseOptions {
        CloseOptions { 
        channel: channel.into(),
             ..Default::default()
        }
    }
}

impl From<CloseOptions> for String {
    fn from(options: CloseOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The CloseBuilder provides a fluid interface to create
/// objects of type CloseOptions
pub struct CloseBuilder<'a> {
    client: &'a mut Client,
    params: CloseOptions,
}

impl<'a> CloseBuilder<'a> {

    /// Create a default CloseBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, channel: S) -> CloseBuilder<'a> {
        CloseBuilder {
            client: client,
            params: CloseOptions::new(
            channel,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a CloseResponse
    pub fn send(&mut self) -> Result<MpimCloseResponse> {
        let payload = self.client.send(format!("{}?{:?}", "mpim.close", &self.params))?;
        payload.to_type::<MpimCloseResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn mpim_close<S: Into<String>>(&mut self, channel: S) -> CloseBuilder {
        CloseBuilder::default(self,
        channel.into(),
        )
    }
}