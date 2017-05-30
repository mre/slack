//! # auth_revoke
//!
//! This method revokes an access token. Use it when you no longer need a token. For example, with a Sign In With Slack app, call this to log a user out.  
//! See https://api.slack.com/methods/auth.revoke  
//!

//!
//! ## Optional arguments: 
//!
//!  `test` (bool)  
//!  Setting this parameter to 1 triggers a testing mode where the specified token will not actually be revoked.  
//!  Example: true

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
pub struct RevokeOptions {

/// Setting this parameter to 1 triggers a testing mode where the specified token will not actually be revoked.
    pub test: Option<bool>,
}

impl<'a> RevokeOptions {

    /// Create a new instance of RevokeOptions 
    fn new<>() -> RevokeOptions {
        RevokeOptions { 
        
             ..Default::default()
        }
    }
}

impl From<RevokeOptions> for String {
    fn from(options: RevokeOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The RevokeBuilder provides a fluid interface to create
/// objects of type RevokeOptions
pub struct RevokeBuilder<'a> {
    client: &'a mut Client,
    params: RevokeOptions,
}

impl<'a> RevokeBuilder<'a> {

    /// Create a default RevokeBuilder object
    pub fn default<>(client: &'a mut Client, ) -> RevokeBuilder<'a> {
        RevokeBuilder {
            client: client,
            params: RevokeOptions::new(
            
            ),
        }
    }
/// Setting this parameter to 1 triggers a testing mode where the specified token will not actually be revoked.
    pub fn test(&'a mut self, test: bool) -> &mut RevokeBuilder {
        self.params.test = Some(test);
        self
    }

    /// Send the request to Slack and try to convert the response to a RevokeResponse
    pub fn send(&mut self) -> Result<AuthRevokeResponse> {
        let payload = self.client.send(format!("{}?{:?}", "auth.revoke", &self.params))?;
        payload.to_type::<AuthRevokeResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn auth_revoke<>(&mut self, ) -> RevokeBuilder {
        RevokeBuilder::default(self,
        
        )
    }
}