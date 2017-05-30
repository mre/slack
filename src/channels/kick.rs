//! # channels_kick
//!
//! This method allows a user to remove another member from a team channel.  
//! See https://api.slack.com/methods/channels.kick  
//!
//! ## Required arguments: 
//!  `channel` (String)  
//!  Channel to remove user from.  
//!  Example: C1234567890  
//!  `user` (String)  
//!  User to remove from channel.  
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
pub struct KickOptions {
/// Channel to remove user from.
    pub channel: String,
/// User to remove from channel.
    pub user: String,

}

impl<'a> KickOptions {

    /// Create a new instance of KickOptions 
    fn new<S: Into<String>>(channel: S, user: S) -> KickOptions {
        KickOptions { 
        channel: channel.into(),
        user: user.into(),
             ..Default::default()
        }
    }
}

impl From<KickOptions> for String {
    fn from(options: KickOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The KickBuilder provides a fluid interface to create
/// objects of type KickOptions
pub struct KickBuilder<'a> {
    client: &'a mut Client,
    params: KickOptions,
}

impl<'a> KickBuilder<'a> {

    /// Create a default KickBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, channel: S, user: S) -> KickBuilder<'a> {
        KickBuilder {
            client: client,
            params: KickOptions::new(
            channel,
            user,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a KickResponse
    pub fn send(&mut self) -> Result<ChannelsKickResponse> {
        let payload = self.client.send(format!("{}?{:?}", "channels.kick", &self.params))?;
        payload.to_type::<ChannelsKickResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn channels_kick<S: Into<String>>(&mut self, channel: S, user: S) -> KickBuilder {
        KickBuilder::default(self,
        channel.into(),
        user.into(),
        )
    }
}