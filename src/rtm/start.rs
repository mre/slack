//! # rtm_start
//!
//! This method starts a Real Time Messaging API session. Refer to the  
//! RTM API documentation for full details on how to use the RTM API.  
//! See https://api.slack.com/methods/rtm.start  
//!

//!
//! ## Optional arguments: 
//!
//!  `mpim_aware` (String)  
//!  Returns MPIMs to the client in the API response.  
//!  Example:    
//!
//!  `no_unreads` (String)  
//!  Skip unread counts for each channel (improves performance).  
//!  Example:    
//!
//!  `simple_latest` (String)  
//!  Return timestamp only for latest message object of each channel (improves performance).  
//!  Example:

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
pub struct StartOptions {

/// Returns MPIMs to the client in the API response.
    pub mpim_aware: Option<String>,
/// Skip unread counts for each channel (improves performance).
    pub no_unreads: Option<String>,
/// Return timestamp only for latest message object of each channel (improves performance).
    pub simple_latest: Option<String>,
}

impl<'a> StartOptions {

    /// Create a new instance of StartOptions 
    fn new<>() -> StartOptions {
        StartOptions { 
        
             ..Default::default()
        }
    }
}

impl From<StartOptions> for String {
    fn from(options: StartOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The StartBuilder provides a fluid interface to create
/// objects of type StartOptions
pub struct StartBuilder<'a> {
    client: &'a mut Client,
    params: StartOptions,
}

impl<'a> StartBuilder<'a> {

    /// Create a default StartBuilder object
    pub fn default<>(client: &'a mut Client, ) -> StartBuilder<'a> {
        StartBuilder {
            client: client,
            params: StartOptions::new(
            
            ),
        }
    }
/// Returns MPIMs to the client in the API response.
    pub fn mpim_aware<S: Into<String>>(&'a mut self, mpim_aware: S) -> &mut StartBuilder {
        self.params.mpim_aware = Some(mpim_aware.into());
        self
    }
/// Skip unread counts for each channel (improves performance).
    pub fn no_unreads<S: Into<String>>(&'a mut self, no_unreads: S) -> &mut StartBuilder {
        self.params.no_unreads = Some(no_unreads.into());
        self
    }
/// Return timestamp only for latest message object of each channel (improves performance).
    pub fn simple_latest<S: Into<String>>(&'a mut self, simple_latest: S) -> &mut StartBuilder {
        self.params.simple_latest = Some(simple_latest.into());
        self
    }

    /// Send the request to Slack and try to convert the response to a StartResponse
    pub fn send(&mut self) -> Result<RtmStartResponse> {
        let payload = self.client.send(format!("{}?{:?}", "rtm.start", &self.params))?;
        payload.to_type::<RtmStartResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn rtm_start<>(&mut self, ) -> StartBuilder {
        StartBuilder::default(self,
        
        )
    }
}