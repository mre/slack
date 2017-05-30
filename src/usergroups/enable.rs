//! # usergroups_enable
//!
//! This method enables a User Group which was previously disabled.  
//! See https://api.slack.com/methods/usergroups.enable  
//!
//! ## Required arguments: 
//!  `usergroup` (String)  
//!  The encoded ID of the User Group to enable.  
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
pub struct EnableOptions {
/// The encoded ID of the User Group to enable.
    pub usergroup: String,
/// Include the number of users in the User Group.
    pub include_count: Option<bool>,
}

impl<'a> EnableOptions {

    /// Create a new instance of EnableOptions 
    fn new<S: Into<String>>(usergroup: S) -> EnableOptions {
        EnableOptions { 
        usergroup: usergroup.into(),
             ..Default::default()
        }
    }
}

impl From<EnableOptions> for String {
    fn from(options: EnableOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The EnableBuilder provides a fluid interface to create
/// objects of type EnableOptions
pub struct EnableBuilder<'a> {
    client: &'a mut Client,
    params: EnableOptions,
}

impl<'a> EnableBuilder<'a> {

    /// Create a default EnableBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, usergroup: S) -> EnableBuilder<'a> {
        EnableBuilder {
            client: client,
            params: EnableOptions::new(
            usergroup,
            ),
        }
    }
/// Include the number of users in the User Group.
    pub fn include_count(&'a mut self, include_count: bool) -> &mut EnableBuilder {
        self.params.include_count = Some(include_count);
        self
    }

    /// Send the request to Slack and try to convert the response to a EnableResponse
    pub fn send(&mut self) -> Result<UsergroupsEnableResponse> {
        let payload = self.client.send(format!("{}?{:?}", "usergroups.enable", &self.params))?;
        payload.to_type::<UsergroupsEnableResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn usergroups_enable<S: Into<String>>(&mut self, usergroup: S) -> EnableBuilder {
        EnableBuilder::default(self,
        usergroup.into(),
        )
    }
}