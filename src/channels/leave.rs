//! # channels_leave
//!
//! This method is used to leave a channel.  
//! See https://api.slack.com/methods/channels.leave  
//!
//! ## Required arguments: 
//!  `channel` (String)  
//!  Channel to leave.  
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
pub struct LeaveOptions {
/// Channel to leave.
    pub channel: String,

}

impl<'a> LeaveOptions {

    /// Create a new instance of LeaveOptions 
    fn new<S: Into<String>>(channel: S) -> LeaveOptions {
        LeaveOptions { 
        channel: channel.into(),
             ..Default::default()
        }
    }
}

impl From<LeaveOptions> for String {
    fn from(options: LeaveOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The LeaveBuilder provides a fluid interface to create
/// objects of type LeaveOptions
pub struct LeaveBuilder<'a> {
    client: &'a mut Client,
    params: LeaveOptions,
}

impl<'a> LeaveBuilder<'a> {

    /// Create a default LeaveBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, channel: S) -> LeaveBuilder<'a> {
        LeaveBuilder {
            client: client,
            params: LeaveOptions::new(
            channel,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a LeaveResponse
    pub fn send(&mut self) -> Result<ChannelsLeaveResponse> {
        let payload = self.client.send(format!("{}?{:?}", "channels.leave", &self.params))?;
        payload.to_type::<ChannelsLeaveResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn channels_leave<S: Into<String>>(&mut self, channel: S) -> LeaveBuilder {
        LeaveBuilder::default(self,
        channel.into(),
        )
    }
}