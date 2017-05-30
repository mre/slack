//! # channels_invite
//!
//! This method is used to invite a user to a channel. The calling user must be a member of the channel.  
//! See https://api.slack.com/methods/channels.invite  
//!
//! ## Required arguments: 
//!  `channel` (String)  
//!  Channel to invite user to.  
//!  Example: C1234567890  
//!  `user` (String)  
//!  User to invite to channel.  
//!  Example: U1234567890


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
pub struct InviteOptions {
/// Channel to invite user to.
    pub channel: String,
/// User to invite to channel.
    pub user: String,

}

impl<'a> InviteOptions {

    /// Create a new instance of InviteOptions 
    fn new<S: Into<String>>(channel: S, user: S) -> InviteOptions {
        InviteOptions { 
        channel: channel.into(),
        user: user.into(),
             ..Default::default()
        }
    }
}

impl From<InviteOptions> for String {
    fn from(options: InviteOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The InviteBuilder provides a fluid interface to create
/// objects of type InviteOptions
pub struct InviteBuilder<'a> {
    client: &'a mut Client,
    params: InviteOptions,
}

impl<'a> InviteBuilder<'a> {

    /// Create a default InviteBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, channel: S, user: S) -> InviteBuilder<'a> {
        InviteBuilder {
            client: client,
            params: InviteOptions::new(
            channel,
            user,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a InviteResponse
    pub fn send(&mut self) -> Result<ChannelsInviteResponse> {
        let payload = self.client.send(format!("{}?{:?}", "channels.invite", &self.params))?;
        payload.to_type::<ChannelsInviteResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn channels_invite<S: Into<String>>(&mut self, channel: S, user: S) -> InviteBuilder {
        InviteBuilder::default(self,
        channel.into(),
        user.into(),
        )
    }
}