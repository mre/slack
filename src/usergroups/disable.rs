//! # usergroups_disable
//!
//! This method disables an existing User Group.  
//! See https://api.slack.com/methods/usergroups.disable  
//!
//! ## Required arguments: 
//!  `usergroup` (String)  
//!  The encoded ID of the User Group to disable.  
//!  Example: S0604QSJC
//!
//! ## Optional arguments: 
//!
//!  `include_count` (bool)  
//!  Include the number of users in the User Group.  
//!  Example: 1

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
pub struct DisableOptions {
/// The encoded ID of the User Group to disable.
    pub usergroup: String,
/// Include the number of users in the User Group.
    pub include_count: Option<bool>,
}

impl<'a> DisableOptions {

    /// Create a new instance of DisableOptions 
    fn new<S: Into<String>>(usergroup: S) -> DisableOptions {
        DisableOptions { 
        usergroup: usergroup.into(),
             ..Default::default()
        }
    }
}

impl From<DisableOptions> for String {
    fn from(options: DisableOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The DisableBuilder provides a fluid interface to create
/// objects of type DisableOptions
pub struct DisableBuilder<'a> {
    client: &'a mut Client,
    params: DisableOptions,
}

impl<'a> DisableBuilder<'a> {

    /// Create a default DisableBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, usergroup: S) -> DisableBuilder<'a> {
        DisableBuilder {
            client: client,
            params: DisableOptions::new(
            usergroup,
            ),
        }
    }
/// Include the number of users in the User Group.
    pub fn include_count(&'a mut self, include_count: bool) -> &mut DisableBuilder {
        self.params.include_count = Some(include_count);
        self
    }

    /// Send the request to Slack and try to convert the response to a DisableResponse
    pub fn send(&mut self) -> Result<UsergroupsDisableResponse> {
        let payload = self.client.send(format!("{}?{:?}", "usergroups.disable", &self.params))?;
        payload.to_type::<UsergroupsDisableResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn usergroups_disable<S: Into<String>>(&mut self, usergroup: S) -> DisableBuilder {
        DisableBuilder::default(self,
        usergroup.into(),
        )
    }
}