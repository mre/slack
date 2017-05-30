//! # reminders_info
//!
//! This method returns information about a reminder.  
//! See https://api.slack.com/methods/reminders.info  
//!
//! ## Required arguments: 
//!  `reminder` (String)  
//!  The ID of the reminder.  
//!  Example: Rm23456789


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
pub struct InfoOptions {
/// The ID of the reminder.
    pub reminder: String,

}

impl<'a> InfoOptions {

    /// Create a new instance of InfoOptions 
    fn new<S: Into<String>>(reminder: S) -> InfoOptions {
        InfoOptions { 
        reminder: reminder.into(),
             ..Default::default()
        }
    }
}

impl From<InfoOptions> for String {
    fn from(options: InfoOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The InfoBuilder provides a fluid interface to create
/// objects of type InfoOptions
pub struct InfoBuilder<'a> {
    client: &'a mut Client,
    params: InfoOptions,
}

impl<'a> InfoBuilder<'a> {

    /// Create a default InfoBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, reminder: S) -> InfoBuilder<'a> {
        InfoBuilder {
            client: client,
            params: InfoOptions::new(
            reminder,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a InfoResponse
    pub fn send(&mut self) -> Result<RemindersInfoResponse> {
        let payload = self.client.send(format!("{}?{:?}", "reminders.info", &self.params))?;
        payload.to_type::<RemindersInfoResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn reminders_info<S: Into<String>>(&mut self, reminder: S) -> InfoBuilder {
        InfoBuilder::default(self,
        reminder.into(),
        )
    }
}