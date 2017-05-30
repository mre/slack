//! # reminders_add
//!
//! This method creates a reminder.  
//! See https://api.slack.com/methods/reminders.add  
//!
//! ## Required arguments: 
//!  `text` (String)  
//!  The content of the reminder.  
//!  Example: eat a banana  
//!  `time` (i64)  
//!  When this reminder should happen: the Unix timestamp (up to five years from now), the number of seconds until the reminder (if within 24 hours), or a natural language description (Ex. "in 15 minutes," or "every Thursday").  
//!  Example: 1602288000
//!
//! ## Optional arguments: 
//!
//!  `user` (String)  
//!  The user who will receive the reminder. If no user is specified, the reminder will go to user who created it.  
//!  Example: U18888888

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
pub struct AddOptions {
/// The content of the reminder.
    pub text: String,
/// When this reminder should happen: the Unix timestamp (up to five years from now), the number of seconds until the reminder (if within 24 hours), or a natural language description (Ex. "in 15 minutes," or "every Thursday").
    pub time: i64,
/// The user who will receive the reminder. If no user is specified, the reminder will go to user who created it.
    pub user: Option<String>,
}

impl<'a> AddOptions {

    /// Create a new instance of AddOptions 
    fn new<S: Into<String>>(text: S, time: i64) -> AddOptions {
        AddOptions { 
        text: text.into(),
        time: time,
             ..Default::default()
        }
    }
}

impl From<AddOptions> for String {
    fn from(options: AddOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The AddBuilder provides a fluid interface to create
/// objects of type AddOptions
pub struct AddBuilder<'a> {
    client: &'a mut Client,
    params: AddOptions,
}

impl<'a> AddBuilder<'a> {

    /// Create a default AddBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, text: S, time: i64) -> AddBuilder<'a> {
        AddBuilder {
            client: client,
            params: AddOptions::new(
            text,
            time,
            ),
        }
    }
/// The user who will receive the reminder. If no user is specified, the reminder will go to user who created it.
    pub fn user<S: Into<String>>(&'a mut self, user: S) -> &mut AddBuilder {
        self.params.user = Some(user.into());
        self
    }

    /// Send the request to Slack and try to convert the response to a AddResponse
    pub fn send(&mut self) -> Result<RemindersAddResponse> {
        let payload = self.client.send(format!("{}?{:?}", "reminders.add", &self.params))?;
        payload.to_type::<RemindersAddResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn reminders_add<S: Into<String>>(&mut self, text: S, time: i64) -> AddBuilder {
        AddBuilder::default(self,
        text.into(),
        time,
        )
    }
}