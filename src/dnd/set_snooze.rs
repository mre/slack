//! # dnd_set_snooze
//!
//! Adjusts the snooze duration for a user's Do Not Disturb settings. If a snooze session is not already active for the user, invoking this method will begin one for the specified duration.  
//! See https://api.slack.com/methods/dnd.setSnooze  
//!
//! ## Required arguments: 
//!  `num_minutes` (i64)  
//!  Number of minutes, from now, to snooze until.  
//!  Example: 60


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
pub struct SetSnoozeOptions {
/// Number of minutes, from now, to snooze until.
    pub num_minutes: i64,

}

impl<'a> SetSnoozeOptions {

    /// Create a new instance of SetSnoozeOptions 
    fn new<>(num_minutes: i64) -> SetSnoozeOptions {
        SetSnoozeOptions { 
        num_minutes: num_minutes,
             ..Default::default()
        }
    }
}

impl From<SetSnoozeOptions> for String {
    fn from(options: SetSnoozeOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The SetSnoozeBuilder provides a fluid interface to create
/// objects of type SetSnoozeOptions
pub struct SetSnoozeBuilder<'a> {
    client: &'a mut Client,
    params: SetSnoozeOptions,
}

impl<'a> SetSnoozeBuilder<'a> {

    /// Create a default SetSnoozeBuilder object
    pub fn default<>(client: &'a mut Client, num_minutes: i64) -> SetSnoozeBuilder<'a> {
        SetSnoozeBuilder {
            client: client,
            params: SetSnoozeOptions::new(
            num_minutes,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a SetSnoozeResponse
    pub fn send(&mut self) -> Result<DndSetSnoozeResponse> {
        let payload = self.client.send(format!("{}?{:?}", "dnd.setSnooze", &self.params))?;
        payload.to_type::<DndSetSnoozeResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn dnd_set_snooze<>(&mut self, num_minutes: i64) -> SetSnoozeBuilder {
        SetSnoozeBuilder::default(self,
        num_minutes,
        )
    }
}