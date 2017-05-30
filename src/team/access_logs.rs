//! # team_access_logs
//!
//! This method is used to get the access logs for users on a team.  
//! See https://api.slack.com/methods/team.accessLogs  
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
pub struct AccessLogsOptions {


}

impl<'a> AccessLogsOptions {

    /// Create a new instance of AccessLogsOptions 
    fn new<>() -> AccessLogsOptions {
        AccessLogsOptions { 
        
             ..Default::default()
        }
    }
}

impl From<AccessLogsOptions> for String {
    fn from(options: AccessLogsOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The AccessLogsBuilder provides a fluid interface to create
/// objects of type AccessLogsOptions
pub struct AccessLogsBuilder<'a> {
    client: &'a mut Client,
    params: AccessLogsOptions,
}

impl<'a> AccessLogsBuilder<'a> {

    /// Create a default AccessLogsBuilder object
    pub fn default<>(client: &'a mut Client, ) -> AccessLogsBuilder<'a> {
        AccessLogsBuilder {
            client: client,
            params: AccessLogsOptions::new(
            
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a AccessLogsResponse
    pub fn send(&mut self) -> Result<TeamAccessLogsResponse> {
        let payload = self.client.send(format!("{}?{:?}", "team.accessLogs", &self.params))?;
        payload.to_type::<TeamAccessLogsResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn team_access_logs<>(&mut self, ) -> AccessLogsBuilder {
        AccessLogsBuilder::default(self,
        
        )
    }
}