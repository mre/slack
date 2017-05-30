//! # users_set_active
//!
//! This method lets the slack messaging server know that the authenticated user  
//! is currently active. Consult the presence documentation for  
//! more details.  
//! See https://api.slack.com/methods/users.setActive  
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
pub struct SetActiveOptions {


}

impl<'a> SetActiveOptions {

    /// Create a new instance of SetActiveOptions 
    fn new<>() -> SetActiveOptions {
        SetActiveOptions { 
        
             ..Default::default()
        }
    }
}

impl From<SetActiveOptions> for String {
    fn from(options: SetActiveOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The SetActiveBuilder provides a fluid interface to create
/// objects of type SetActiveOptions
pub struct SetActiveBuilder<'a> {
    client: &'a mut Client,
    params: SetActiveOptions,
}

impl<'a> SetActiveBuilder<'a> {

    /// Create a default SetActiveBuilder object
    pub fn default<>(client: &'a mut Client, ) -> SetActiveBuilder<'a> {
        SetActiveBuilder {
            client: client,
            params: SetActiveOptions::new(
            
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a SetActiveResponse
    pub fn send(&mut self) -> Result<UsersSetActiveResponse> {
        let payload = self.client.send(format!("{}?{:?}", "users.setActive", &self.params))?;
        payload.to_type::<UsersSetActiveResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn users_set_active<>(&mut self, ) -> SetActiveBuilder {
        SetActiveBuilder::default(self,
        
        )
    }
}