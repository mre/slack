//! # reminders_complete
//!
//! This method completes a reminder.  
//! See https://api.slack.com/methods/reminders.complete  
//!
//! ## Required arguments: 
//!  `reminder` (String)  
//!  The ID of the reminder to be marked as complete.  
//!  Example: Rm12345678


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
pub struct CompleteOptions {
/// The ID of the reminder to be marked as complete.
    pub reminder: String,

}

impl<'a> CompleteOptions {

    /// Create a new instance of CompleteOptions 
    fn new<S: Into<String>>(reminder: S) -> CompleteOptions {
        CompleteOptions { 
        reminder: reminder.into(),
             ..Default::default()
        }
    }
}

impl From<CompleteOptions> for String {
    fn from(options: CompleteOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The CompleteBuilder provides a fluid interface to create
/// objects of type CompleteOptions
pub struct CompleteBuilder<'a> {
    client: &'a mut Client,
    params: CompleteOptions,
}

impl<'a> CompleteBuilder<'a> {

    /// Create a default CompleteBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, reminder: S) -> CompleteBuilder<'a> {
        CompleteBuilder {
            client: client,
            params: CompleteOptions::new(
            reminder,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a CompleteResponse
    pub fn send(&mut self) -> Result<RemindersCompleteResponse> {
        let payload = self.client.send(format!("{}?{:?}", "reminders.complete", &self.params))?;
        payload.to_type::<RemindersCompleteResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn reminders_complete<S: Into<String>>(&mut self, reminder: S) -> CompleteBuilder {
        CompleteBuilder::default(self,
        reminder.into(),
        )
    }
}