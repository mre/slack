//! # team_integration_logs
//!
//! This method lists the integration activity logs for a team, including when integrations are added, modified and removed. This method can only be called by Admins.  
//! See https://api.slack.com/methods/team.integrationLogs  
//!

//!
//! ## Optional arguments: 
//!
//!  `app_id` (String)  
//!  Filter logs to this Slack app. Defaults to all logs.  
//!  Example:    
//!
//!  `change_type` (String)  
//!  Filter logs with this change type. Defaults to all logs.  
//!  Example: added  
//!
//!  `service_id` (String)  
//!  Filter logs to this service. Defaults to all logs.  
//!  Example:    
//!
//!  `user` (String)  
//!  Filter logs generated by this user's actions. Defaults to all logs.  
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
pub struct IntegrationLogsOptions {

/// Filter logs to this Slack app. Defaults to all logs.
    pub app_id: Option<String>,
/// Filter logs with this change type. Defaults to all logs.
    pub change_type: Option<String>,
/// Filter logs to this service. Defaults to all logs.
    pub service_id: Option<String>,
/// Filter logs generated by this user's actions. Defaults to all logs.
    pub user: Option<String>,
}

impl<'a> IntegrationLogsOptions {

    /// Create a new instance of IntegrationLogsOptions 
    fn new<>() -> IntegrationLogsOptions {
        IntegrationLogsOptions { 
        
             ..Default::default()
        }
    }
}

impl From<IntegrationLogsOptions> for String {
    fn from(options: IntegrationLogsOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The IntegrationLogsBuilder provides a fluid interface to create
/// objects of type IntegrationLogsOptions
pub struct IntegrationLogsBuilder<'a> {
    client: &'a mut Client,
    params: IntegrationLogsOptions,
}

impl<'a> IntegrationLogsBuilder<'a> {

    /// Create a default IntegrationLogsBuilder object
    pub fn default<>(client: &'a mut Client, ) -> IntegrationLogsBuilder<'a> {
        IntegrationLogsBuilder {
            client: client,
            params: IntegrationLogsOptions::new(
            
            ),
        }
    }
/// Filter logs to this Slack app. Defaults to all logs.
    pub fn app_id<S: Into<String>>(&'a mut self, app_id: S) -> &mut IntegrationLogsBuilder {
        self.params.app_id = Some(app_id.into());
        self
    }
/// Filter logs with this change type. Defaults to all logs.
    pub fn change_type<S: Into<String>>(&'a mut self, change_type: S) -> &mut IntegrationLogsBuilder {
        self.params.change_type = Some(change_type.into());
        self
    }
/// Filter logs to this service. Defaults to all logs.
    pub fn service_id<S: Into<String>>(&'a mut self, service_id: S) -> &mut IntegrationLogsBuilder {
        self.params.service_id = Some(service_id.into());
        self
    }
/// Filter logs generated by this user's actions. Defaults to all logs.
    pub fn user<S: Into<String>>(&'a mut self, user: S) -> &mut IntegrationLogsBuilder {
        self.params.user = Some(user.into());
        self
    }

    /// Send the request to Slack and try to convert the response to a IntegrationLogsResponse
    pub fn send(&mut self) -> Result<TeamIntegrationLogsResponse> {
        let payload = self.client.send(format!("{}?{:?}", "team.integrationLogs", &self.params))?;
        payload.to_type::<TeamIntegrationLogsResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn team_integration_logs<>(&mut self, ) -> IntegrationLogsBuilder {
        IntegrationLogsBuilder::default(self,
        
        )
    }
}