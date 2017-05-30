//! # reminders_delete
//!
//! This method deletes a reminder.  
//! See https://api.slack.com/methods/reminders.delete  
//!
//! ## Required arguments: 
//!  `reminder` (String)  
//!  The ID of the reminder.  
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
pub struct DeleteOptions {
/// The ID of the reminder.
    pub reminder: String,

}

impl<'a> DeleteOptions {

    /// Create a new instance of DeleteOptions 
    fn new<S: Into<String>>(reminder: S) -> DeleteOptions {
        DeleteOptions { 
        reminder: reminder.into(),
             ..Default::default()
        }
    }
}

impl From<DeleteOptions> for String {
    fn from(options: DeleteOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The DeleteBuilder provides a fluid interface to create
/// objects of type DeleteOptions
pub struct DeleteBuilder<'a> {
    client: &'a mut Client,
    params: DeleteOptions,
}

impl<'a> DeleteBuilder<'a> {

    /// Create a default DeleteBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, reminder: S) -> DeleteBuilder<'a> {
        DeleteBuilder {
            client: client,
            params: DeleteOptions::new(
            reminder,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a DeleteResponse
    pub fn send(&mut self) -> Result<RemindersDeleteResponse> {
        let payload = self.client.send(format!("{}?{:?}", "reminders.delete", &self.params))?;
        payload.to_type::<RemindersDeleteResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn reminders_delete<S: Into<String>>(&mut self, reminder: S) -> DeleteBuilder {
        DeleteBuilder::default(self,
        reminder.into(),
        )
    }
}