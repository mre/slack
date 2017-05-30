//! # users_profile_set
//!
//! This method is used to set the profile information for a user.  
//! See https://api.slack.com/methods/users.profile.set  
//!

//!
//! ## Optional arguments: 
//!
//!  `name` (String)  
//!  Name of a single key to set. Usable only if profile is not passed.  
//!  Example: first_name  
//!
//!  `profile` (String)  
//!  Collection of key:value pairs presented as a URL-encoded JSON hash.  
//!  Example: { first_name: "John", ... }  
//!
//!  `user` (String)  
//!  ID of user to change. This argument may only be specified by team admins.  
//!  Example: U1234567890  
//!
//!  `value` (String)  
//!  Value to set a single key to. Usable only if profile is not passed.  
//!  Example: John

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
pub struct ProfileSetOptions {

/// Name of a single key to set. Usable only if profile is not passed.
    pub name: Option<String>,
/// Collection of key:value pairs presented as a URL-encoded JSON hash.
    pub profile: Option<String>,
/// ID of user to change. This argument may only be specified by team admins.
    pub user: Option<String>,
/// Value to set a single key to. Usable only if profile is not passed.
    pub value: Option<String>,
}

impl<'a> ProfileSetOptions {

    /// Create a new instance of ProfileSetOptions 
    fn new<>() -> ProfileSetOptions {
        ProfileSetOptions { 
        
             ..Default::default()
        }
    }
}

impl From<ProfileSetOptions> for String {
    fn from(options: ProfileSetOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The ProfileSetBuilder provides a fluid interface to create
/// objects of type ProfileSetOptions
pub struct ProfileSetBuilder<'a> {
    client: &'a mut Client,
    params: ProfileSetOptions,
}

impl<'a> ProfileSetBuilder<'a> {

    /// Create a default ProfileSetBuilder object
    pub fn default<>(client: &'a mut Client, ) -> ProfileSetBuilder<'a> {
        ProfileSetBuilder {
            client: client,
            params: ProfileSetOptions::new(
            
            ),
        }
    }
/// Name of a single key to set. Usable only if profile is not passed.
    pub fn name<S: Into<String>>(&'a mut self, name: S) -> &mut ProfileSetBuilder {
        self.params.name = Some(name.into());
        self
    }
/// Collection of key:value pairs presented as a URL-encoded JSON hash.
    pub fn profile<S: Into<String>>(&'a mut self, profile: S) -> &mut ProfileSetBuilder {
        self.params.profile = Some(profile.into());
        self
    }
/// ID of user to change. This argument may only be specified by team admins.
    pub fn user<S: Into<String>>(&'a mut self, user: S) -> &mut ProfileSetBuilder {
        self.params.user = Some(user.into());
        self
    }
/// Value to set a single key to. Usable only if profile is not passed.
    pub fn value<S: Into<String>>(&'a mut self, value: S) -> &mut ProfileSetBuilder {
        self.params.value = Some(value.into());
        self
    }

    /// Send the request to Slack and try to convert the response to a ProfileSetResponse
    pub fn send(&mut self) -> Result<UsersProfileSetResponse> {
        let payload = self.client.send(format!("{}?{:?}", "users.profile.set", &self.params))?;
        payload.to_type::<UsersProfileSetResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn users_profile_set<>(&mut self, ) -> ProfileSetBuilder {
        ProfileSetBuilder::default(self,
        
        )
    }
}