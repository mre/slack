//! # users_profile_get
//!
//! This method is used to get the profile information for a user.  
//! See https://api.slack.com/methods/users.profile.get  
//!

//!
//! ## Optional arguments: 
//!
//!  `include_labels` (bool)  
//!  Include labels for each ID in custom profile fields.  
//!  Example: 1  
//!
//!  `user` (String)  
//!  User to retrieve profile info for.  
//!  Example: U1234567890

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
pub struct ProfileGetOptions {

/// Include labels for each ID in custom profile fields.
    pub include_labels: Option<bool>,
/// User to retrieve profile info for.
    pub user: Option<String>,
}

impl<'a> ProfileGetOptions {

    /// Create a new instance of ProfileGetOptions 
    fn new<>() -> ProfileGetOptions {
        ProfileGetOptions { 
        
             ..Default::default()
        }
    }
}

impl From<ProfileGetOptions> for String {
    fn from(options: ProfileGetOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The ProfileGetBuilder provides a fluid interface to create
/// objects of type ProfileGetOptions
pub struct ProfileGetBuilder<'a> {
    client: &'a mut Client,
    params: ProfileGetOptions,
}

impl<'a> ProfileGetBuilder<'a> {

    /// Create a default ProfileGetBuilder object
    pub fn default<>(client: &'a mut Client, ) -> ProfileGetBuilder<'a> {
        ProfileGetBuilder {
            client: client,
            params: ProfileGetOptions::new(
            
            ),
        }
    }
/// Include labels for each ID in custom profile fields.
    pub fn include_labels(&'a mut self, include_labels: bool) -> &mut ProfileGetBuilder {
        self.params.include_labels = Some(include_labels);
        self
    }
/// User to retrieve profile info for.
    pub fn user<S: Into<String>>(&'a mut self, user: S) -> &mut ProfileGetBuilder {
        self.params.user = Some(user.into());
        self
    }

    /// Send the request to Slack and try to convert the response to a ProfileGetResponse
    pub fn send(&mut self) -> Result<UsersProfileGetResponse> {
        let payload = self.client.send(format!("{}?{:?}", "users.profile.get", &self.params))?;
        payload.to_type::<UsersProfileGetResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn users_profile_get<>(&mut self, ) -> ProfileGetBuilder {
        ProfileGetBuilder::default(self,
        
        )
    }
}