//! # dnd_info
//!
//! Provides information about a user's current Do Not Disturb settings.  
//! See https://api.slack.com/methods/dnd.info  
//!

//!
//! ## Optional arguments: 
//!
//!  `user` (String)  
//!  User to fetch status for (defaults to current user).  
//!  Example: U1234

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

/// User to fetch status for (defaults to current user).
    pub user: Option<String>,
}

impl<'a> InfoOptions {

    /// Create a new instance of InfoOptions 
    fn new<>() -> InfoOptions {
        InfoOptions { 
        
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
    pub fn default<>(client: &'a mut Client, ) -> InfoBuilder<'a> {
        InfoBuilder {
            client: client,
            params: InfoOptions::new(
            
            ),
        }
    }
/// User to fetch status for (defaults to current user).
    pub fn user<S: Into<String>>(&'a mut self, user: S) -> &mut InfoBuilder {
        self.params.user = Some(user.into());
        self
    }

    /// Send the request to Slack and try to convert the response to a InfoResponse
    pub fn send(&mut self) -> Result<DndInfoResponse> {
        let payload = self.client.send(format!("{}?{:?}", "dnd.info", &self.params))?;
        payload.to_type::<DndInfoResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn dnd_info<>(&mut self, ) -> InfoBuilder {
        InfoBuilder::default(self,
        
        )
    }
}