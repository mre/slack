//! # dnd_team_info
//!
//! Provides information about the current Do Not Disturb settings for users of a Slack team.  
//! See https://api.slack.com/methods/dnd.teamInfo  
//!

//!
//! ## Optional arguments: 
//!
//!  `users` (String)  
//!  Comma-separated list of users to fetch Do Not Disturb status for.  
//!  Example: U1234,U4567

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
pub struct TeamInfoOptions {

/// Comma-separated list of users to fetch Do Not Disturb status for.
    pub users: Option<String>,
}

impl<'a> TeamInfoOptions {

    /// Create a new instance of TeamInfoOptions 
    fn new<>() -> TeamInfoOptions {
        TeamInfoOptions { 
        
             ..Default::default()
        }
    }
}

impl From<TeamInfoOptions> for String {
    fn from(options: TeamInfoOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The TeamInfoBuilder provides a fluid interface to create
/// objects of type TeamInfoOptions
pub struct TeamInfoBuilder<'a> {
    client: &'a mut Client,
    params: TeamInfoOptions,
}

impl<'a> TeamInfoBuilder<'a> {

    /// Create a default TeamInfoBuilder object
    pub fn default<>(client: &'a mut Client, ) -> TeamInfoBuilder<'a> {
        TeamInfoBuilder {
            client: client,
            params: TeamInfoOptions::new(
            
            ),
        }
    }
/// Comma-separated list of users to fetch Do Not Disturb status for.
    pub fn users<S: Into<String>>(&'a mut self, users: S) -> &mut TeamInfoBuilder {
        self.params.users = Some(users.into());
        self
    }

    /// Send the request to Slack and try to convert the response to a TeamInfoResponse
    pub fn send(&mut self) -> Result<DndTeamInfoResponse> {
        let payload = self.client.send(format!("{}?{:?}", "dnd.teamInfo", &self.params))?;
        payload.to_type::<DndTeamInfoResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn dnd_team_info<>(&mut self, ) -> TeamInfoBuilder {
        TeamInfoBuilder::default(self,
        
        )
    }
}