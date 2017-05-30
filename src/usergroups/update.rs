//! # usergroups_update
//!
//! This method updates the properties of an existing User Group.  
//! See https://api.slack.com/methods/usergroups.update  
//!
//! ## Required arguments: 
//!  `usergroup` (String)  
//!  The encoded ID of the User Group to update.  
//!  Example: S0604QSJC
//!
//! ## Optional arguments: 
//!
//!  `channels` (String)  
//!  A comma separated string of encoded channel IDs for which the User Group uses as a default.  
//!  Example:    
//!
//!  `description` (String)  
//!  A short description of the User Group.  
//!  Example:    
//!
//!  `handle` (String)  
//!  A mention handle. Must be unique among channels, users and User Groups.  
//!  Example:    
//!
//!  `include_count` (bool)  
//!  Include the number of users in the User Group.  
//!  Example: 1  
//!
//!  `name` (String)  
//!  A name for the User Group. Must be unique among User Groups.  
//!  Example: My Test Team

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
pub struct UpdateOptions {
/// The encoded ID of the User Group to update.
    pub usergroup: String,
/// A comma separated string of encoded channel IDs for which the User Group uses as a default.
    pub channels: Option<String>,
/// A short description of the User Group.
    pub description: Option<String>,
/// A mention handle. Must be unique among channels, users and User Groups.
    pub handle: Option<String>,
/// Include the number of users in the User Group.
    pub include_count: Option<bool>,
/// A name for the User Group. Must be unique among User Groups.
    pub name: Option<String>,
}

impl<'a> UpdateOptions {

    /// Create a new instance of UpdateOptions 
    fn new<S: Into<String>>(usergroup: S) -> UpdateOptions {
        UpdateOptions { 
        usergroup: usergroup.into(),
             ..Default::default()
        }
    }
}

impl From<UpdateOptions> for String {
    fn from(options: UpdateOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The UpdateBuilder provides a fluid interface to create
/// objects of type UpdateOptions
pub struct UpdateBuilder<'a> {
    client: &'a mut Client,
    params: UpdateOptions,
}

impl<'a> UpdateBuilder<'a> {

    /// Create a default UpdateBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, usergroup: S) -> UpdateBuilder<'a> {
        UpdateBuilder {
            client: client,
            params: UpdateOptions::new(
            usergroup,
            ),
        }
    }
/// A comma separated string of encoded channel IDs for which the User Group uses as a default.
    pub fn channels<S: Into<String>>(&'a mut self, channels: S) -> &mut UpdateBuilder {
        self.params.channels = Some(channels.into());
        self
    }
/// A short description of the User Group.
    pub fn description<S: Into<String>>(&'a mut self, description: S) -> &mut UpdateBuilder {
        self.params.description = Some(description.into());
        self
    }
/// A mention handle. Must be unique among channels, users and User Groups.
    pub fn handle<S: Into<String>>(&'a mut self, handle: S) -> &mut UpdateBuilder {
        self.params.handle = Some(handle.into());
        self
    }
/// Include the number of users in the User Group.
    pub fn include_count(&'a mut self, include_count: bool) -> &mut UpdateBuilder {
        self.params.include_count = Some(include_count);
        self
    }
/// A name for the User Group. Must be unique among User Groups.
    pub fn name<S: Into<String>>(&'a mut self, name: S) -> &mut UpdateBuilder {
        self.params.name = Some(name.into());
        self
    }

    /// Send the request to Slack and try to convert the response to a UpdateResponse
    pub fn send(&mut self) -> Result<UsergroupsUpdateResponse> {
        let payload = self.client.send(format!("{}?{:?}", "usergroups.update", &self.params))?;
        payload.to_type::<UsergroupsUpdateResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn usergroups_update<S: Into<String>>(&mut self, usergroup: S) -> UpdateBuilder {
        UpdateBuilder::default(self,
        usergroup.into(),
        )
    }
}