//! # users_get_presence
//!
//! This method lets you find out information about a user's presence.  
//! Consult the presence documentation for more details.  
//! See https://api.slack.com/methods/users.getPresence  
//!
//! ## Required arguments: 
//!  `user` (String)  
//!  User to get presence info on. Defaults to the authed user.  
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
pub struct GetPresenceOptions {
/// User to get presence info on. Defaults to the authed user.
    pub user: String,

}

impl<'a> GetPresenceOptions {

    /// Create a new instance of GetPresenceOptions 
    fn new<S: Into<String>>(user: S) -> GetPresenceOptions {
        GetPresenceOptions { 
        user: user.into(),
             ..Default::default()
        }
    }
}

impl From<GetPresenceOptions> for String {
    fn from(options: GetPresenceOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The GetPresenceBuilder provides a fluid interface to create
/// objects of type GetPresenceOptions
pub struct GetPresenceBuilder<'a> {
    client: &'a mut Client,
    params: GetPresenceOptions,
}

impl<'a> GetPresenceBuilder<'a> {

    /// Create a default GetPresenceBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, user: S) -> GetPresenceBuilder<'a> {
        GetPresenceBuilder {
            client: client,
            params: GetPresenceOptions::new(
            user,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a GetPresenceResponse
    pub fn send(&mut self) -> Result<UsersGetPresenceResponse> {
        let payload = self.client.send(format!("{}?{:?}", "users.getPresence", &self.params))?;
        payload.to_type::<UsersGetPresenceResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn users_get_presence<S: Into<String>>(&mut self, user: S) -> GetPresenceBuilder {
        GetPresenceBuilder::default(self,
        user.into(),
        )
    }
}