//! # users_identity
//!
//! After your Slack app is awarded an identity token through Sign in with Slack, use this method to retrieve a user's identity.  
//! See https://api.slack.com/methods/users.identity  
//!



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
pub struct IdentityOptions {


}

impl<'a> IdentityOptions {

    /// Create a new instance of IdentityOptions 
    fn new<>() -> IdentityOptions {
        IdentityOptions { 
        
             ..Default::default()
        }
    }
}

impl From<IdentityOptions> for String {
    fn from(options: IdentityOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The IdentityBuilder provides a fluid interface to create
/// objects of type IdentityOptions
pub struct IdentityBuilder<'a> {
    client: &'a mut Client,
    params: IdentityOptions,
}

impl<'a> IdentityBuilder<'a> {

    /// Create a default IdentityBuilder object
    pub fn default<>(client: &'a mut Client, ) -> IdentityBuilder<'a> {
        IdentityBuilder {
            client: client,
            params: IdentityOptions::new(
            
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a IdentityResponse
    pub fn send(&mut self) -> Result<UsersIdentityResponse> {
        let payload = self.client.send(format!("{}?{:?}", "users.identity", &self.params))?;
        payload.to_type::<UsersIdentityResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn users_identity<>(&mut self, ) -> IdentityBuilder {
        IdentityBuilder::default(self,
        
        )
    }
}