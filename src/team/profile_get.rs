//! # team_profile_get
//!
//! This method is used to get the profile field definitions for this team.  
//! See https://api.slack.com/methods/team.profile.get  
//!

//!
//! ## Optional arguments: 
//!
//!  `visibility` (String)  
//!  Filter by visibility.  
//!  Example: all

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

/// Filter by visibility.
    pub visibility: Option<String>,
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
/// Filter by visibility.
    pub fn visibility<S: Into<String>>(&'a mut self, visibility: S) -> &mut ProfileGetBuilder {
        self.params.visibility = Some(visibility.into());
        self
    }

    /// Send the request to Slack and try to convert the response to a ProfileGetResponse
    pub fn send(&mut self) -> Result<TeamProfileGetResponse> {
        let payload = self.client.send(format!("{}?{:?}", "team.profile.get", &self.params))?;
        payload.to_type::<TeamProfileGetResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn team_profile_get<>(&mut self, ) -> ProfileGetBuilder {
        ProfileGetBuilder::default(self,
        
        )
    }
}