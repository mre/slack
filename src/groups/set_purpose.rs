//! # groups_set_purpose
//!
//! This method is used to change the purpose of a private channel. The calling user must be a member of the private channel.  
//! See https://api.slack.com/methods/groups.setPurpose  
//!
//! ## Required arguments: 
//!  `channel` (String)  
//!  Private channel to set the purpose of.  
//!  Example: C1234567890  
//!  `purpose` (String)  
//!  The new purpose.  
//!  Example: My Purpose


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
pub struct SetPurposeOptions {
/// Private channel to set the purpose of.
    pub channel: String,
/// The new purpose.
    pub purpose: String,

}

impl<'a> SetPurposeOptions {

    /// Create a new instance of SetPurposeOptions 
    fn new<S: Into<String>>(channel: S, purpose: S) -> SetPurposeOptions {
        SetPurposeOptions { 
        channel: channel.into(),
        purpose: purpose.into(),
             ..Default::default()
        }
    }
}

impl From<SetPurposeOptions> for String {
    fn from(options: SetPurposeOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The SetPurposeBuilder provides a fluid interface to create
/// objects of type SetPurposeOptions
pub struct SetPurposeBuilder<'a> {
    client: &'a mut Client,
    params: SetPurposeOptions,
}

impl<'a> SetPurposeBuilder<'a> {

    /// Create a default SetPurposeBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, channel: S, purpose: S) -> SetPurposeBuilder<'a> {
        SetPurposeBuilder {
            client: client,
            params: SetPurposeOptions::new(
            channel,
            purpose,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a SetPurposeResponse
    pub fn send(&mut self) -> Result<GroupsSetPurposeResponse> {
        let payload = self.client.send(format!("{}?{:?}", "groups.setPurpose", &self.params))?;
        payload.to_type::<GroupsSetPurposeResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn groups_set_purpose<S: Into<String>>(&mut self, channel: S, purpose: S) -> SetPurposeBuilder {
        SetPurposeBuilder::default(self,
        channel.into(),
        purpose.into(),
        )
    }
}