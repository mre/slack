//! # users_set_presence
//!
//! This method lets you set the calling user's manual presence.  
//! Consult the presence documentation for more details.  
//! See https://api.slack.com/methods/users.setPresence  
//!
//! ## Required arguments: 
//!  `presence` (String)  
//!  Either auto or away.  
//!  Example: away


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
pub struct SetPresenceOptions {
/// Either auto or away.
    pub presence: String,

}

impl<'a> SetPresenceOptions {

    /// Create a new instance of SetPresenceOptions 
    fn new<S: Into<String>>(presence: S) -> SetPresenceOptions {
        SetPresenceOptions { 
        presence: presence.into(),
             ..Default::default()
        }
    }
}

impl From<SetPresenceOptions> for String {
    fn from(options: SetPresenceOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The SetPresenceBuilder provides a fluid interface to create
/// objects of type SetPresenceOptions
pub struct SetPresenceBuilder<'a> {
    client: &'a mut Client,
    params: SetPresenceOptions,
}

impl<'a> SetPresenceBuilder<'a> {

    /// Create a default SetPresenceBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, presence: S) -> SetPresenceBuilder<'a> {
        SetPresenceBuilder {
            client: client,
            params: SetPresenceOptions::new(
            presence,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a SetPresenceResponse
    pub fn send(&mut self) -> Result<UsersSetPresenceResponse> {
        let payload = self.client.send(format!("{}?{:?}", "users.setPresence", &self.params))?;
        payload.to_type::<UsersSetPresenceResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn users_set_presence<S: Into<String>>(&mut self, presence: S) -> SetPresenceBuilder {
        SetPresenceBuilder::default(self,
        presence.into(),
        )
    }
}