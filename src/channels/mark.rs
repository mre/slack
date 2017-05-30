//! # channels_mark
//!
//! This method moves the read cursor in a channel.  
//! See https://api.slack.com/methods/channels.mark  
//!
//! ## Required arguments: 
//!  `channel` (String)  
//!  Channel to set reading cursor in.  
//!  Example: C1234567890  
//!  `ts` (f64)  
//!  Timestamp of the most recently seen message.  
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
pub struct MarkOptions {
/// Channel to set reading cursor in.
    pub channel: String,
/// Timestamp of the most recently seen message.
    pub ts: f64,

}

impl<'a> MarkOptions {

    /// Create a new instance of MarkOptions 
    fn new<S: Into<String>>(channel: S, ts: f64) -> MarkOptions {
        MarkOptions { 
        channel: channel.into(),
        ts: ts,
             ..Default::default()
        }
    }
}

impl From<MarkOptions> for String {
    fn from(options: MarkOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The MarkBuilder provides a fluid interface to create
/// objects of type MarkOptions
pub struct MarkBuilder<'a> {
    client: &'a mut Client,
    params: MarkOptions,
}

impl<'a> MarkBuilder<'a> {

    /// Create a default MarkBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, channel: S, ts: f64) -> MarkBuilder<'a> {
        MarkBuilder {
            client: client,
            params: MarkOptions::new(
            channel,
            ts,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a MarkResponse
    pub fn send(&mut self) -> Result<ChannelsMarkResponse> {
        let payload = self.client.send(format!("{}?{:?}", "channels.mark", &self.params))?;
        payload.to_type::<ChannelsMarkResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn channels_mark<S: Into<String>>(&mut self, channel: S, ts: f64) -> MarkBuilder {
        MarkBuilder::default(self,
        channel.into(),
        ts,
        )
    }
}