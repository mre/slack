//! # team_billable_info
//!
//! This method lists billable information for each user on the team. Currently this consists solely of whether the user is  
//! subject to billing per Slack's Fair Billing policy.  
//! See https://api.slack.com/methods/team.billableInfo  
//!

//!
//! ## Optional arguments: 
//!
//!  `user` (String)  
//!  A user to retrieve the billable information for. Defaults to all users.  
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
pub struct BillableInfoOptions {

/// A user to retrieve the billable information for. Defaults to all users.
    pub user: Option<String>,
}

impl<'a> BillableInfoOptions {

    /// Create a new instance of BillableInfoOptions 
    fn new<>() -> BillableInfoOptions {
        BillableInfoOptions { 
        
             ..Default::default()
        }
    }
}

impl From<BillableInfoOptions> for String {
    fn from(options: BillableInfoOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The BillableInfoBuilder provides a fluid interface to create
/// objects of type BillableInfoOptions
pub struct BillableInfoBuilder<'a> {
    client: &'a mut Client,
    params: BillableInfoOptions,
}

impl<'a> BillableInfoBuilder<'a> {

    /// Create a default BillableInfoBuilder object
    pub fn default<>(client: &'a mut Client, ) -> BillableInfoBuilder<'a> {
        BillableInfoBuilder {
            client: client,
            params: BillableInfoOptions::new(
            
            ),
        }
    }
/// A user to retrieve the billable information for. Defaults to all users.
    pub fn user<S: Into<String>>(&'a mut self, user: S) -> &mut BillableInfoBuilder {
        self.params.user = Some(user.into());
        self
    }

    /// Send the request to Slack and try to convert the response to a BillableInfoResponse
    pub fn send(&mut self) -> Result<TeamBillableInfoResponse> {
        let payload = self.client.send(format!("{}?{:?}", "team.billableInfo", &self.params))?;
        payload.to_type::<TeamBillableInfoResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn team_billable_info<>(&mut self, ) -> BillableInfoBuilder {
        BillableInfoBuilder::default(self,
        
        )
    }
}