//! # users_info
//!
//! This method returns information about a team member.  
//! See https://api.slack.com/methods/users.info  
//!
//! ## Required arguments: 
//!  `user` (String)  
//!  User to get info on.  
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
pub struct InfoOptions {
/// User to get info on.
    pub user: String,

}

impl<'a> InfoOptions {

    /// Create a new instance of InfoOptions 
    fn new<S: Into<String>>(user: S) -> InfoOptions {
        InfoOptions { 
        user: user.into(),
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
    pub fn default<S: Into<String>>(client: &'a mut Client, user: S) -> InfoBuilder<'a> {
        InfoBuilder {
            client: client,
            params: InfoOptions::new(
            user,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a InfoResponse
    pub fn send(&mut self) -> Result<UsersInfoResponse> {
        let payload = self.client.send(format!("{}?{:?}", "users.info", &self.params))?;
        payload.to_type::<UsersInfoResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn users_info<S: Into<String>>(&mut self, user: S) -> InfoBuilder {
        InfoBuilder::default(self,
        user.into(),
        )
    }
}