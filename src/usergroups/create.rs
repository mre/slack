//! # usergroups_create
//!
//! This method is used to create a User Group.  
//! See https://api.slack.com/methods/usergroups.create  
//!
//! ## Required arguments: 
//!  `name` (String)  
//!  A name for the User Group. Must be unique among User Groups.  
//!  Example: My Test Team
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
//!  Include the number of users in each User Group.  
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
pub struct CreateOptions {
/// A name for the User Group. Must be unique among User Groups.
    pub name: String,
/// A comma separated string of encoded channel IDs for which the User Group uses as a default.
    pub channels: Option<String>,
/// A short description of the User Group.
    pub description: Option<String>,
/// A mention handle. Must be unique among channels, users and User Groups.
    pub handle: Option<String>,
/// Include the number of users in each User Group.
    pub include_count: Option<bool>,
}

impl<'a> CreateOptions {

    /// Create a new instance of CreateOptions 
    fn new<S: Into<String>>(name: S) -> CreateOptions {
        CreateOptions { 
        name: name.into(),
             ..Default::default()
        }
    }
}

impl From<CreateOptions> for String {
    fn from(options: CreateOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The CreateBuilder provides a fluid interface to create
/// objects of type CreateOptions
pub struct CreateBuilder<'a> {
    client: &'a mut Client,
    params: CreateOptions,
}

impl<'a> CreateBuilder<'a> {

    /// Create a default CreateBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, name: S) -> CreateBuilder<'a> {
        CreateBuilder {
            client: client,
            params: CreateOptions::new(
            name,
            ),
        }
    }
/// A comma separated string of encoded channel IDs for which the User Group uses as a default.
    pub fn channels<S: Into<String>>(&'a mut self, channels: S) -> &mut CreateBuilder {
        self.params.channels = Some(channels.into());
        self
    }
/// A short description of the User Group.
    pub fn description<S: Into<String>>(&'a mut self, description: S) -> &mut CreateBuilder {
        self.params.description = Some(description.into());
        self
    }
/// A mention handle. Must be unique among channels, users and User Groups.
    pub fn handle<S: Into<String>>(&'a mut self, handle: S) -> &mut CreateBuilder {
        self.params.handle = Some(handle.into());
        self
    }
/// Include the number of users in each User Group.
    pub fn include_count(&'a mut self, include_count: bool) -> &mut CreateBuilder {
        self.params.include_count = Some(include_count);
        self
    }

    /// Send the request to Slack and try to convert the response to a CreateResponse
    pub fn send(&mut self) -> Result<UsergroupsCreateResponse> {
        let payload = self.client.send(format!("{}?{:?}", "usergroups.create", &self.params))?;
        payload.to_type::<UsergroupsCreateResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn usergroups_create<S: Into<String>>(&mut self, name: S) -> CreateBuilder {
        CreateBuilder::default(self,
        name.into(),
        )
    }
}