//! # dnd_end_snooze
//!
//! Ends the current user's snooze mode immediately.  
//! See https://api.slack.com/methods/dnd.endSnooze  
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
pub struct EndSnoozeOptions {


}

impl<'a> EndSnoozeOptions {

    /// Create a new instance of EndSnoozeOptions 
    fn new<>() -> EndSnoozeOptions {
        EndSnoozeOptions { 
        
             ..Default::default()
        }
    }
}

impl From<EndSnoozeOptions> for String {
    fn from(options: EndSnoozeOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The EndSnoozeBuilder provides a fluid interface to create
/// objects of type EndSnoozeOptions
pub struct EndSnoozeBuilder<'a> {
    client: &'a mut Client,
    params: EndSnoozeOptions,
}

impl<'a> EndSnoozeBuilder<'a> {

    /// Create a default EndSnoozeBuilder object
    pub fn default<>(client: &'a mut Client, ) -> EndSnoozeBuilder<'a> {
        EndSnoozeBuilder {
            client: client,
            params: EndSnoozeOptions::new(
            
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a EndSnoozeResponse
    pub fn send(&mut self) -> Result<DndEndSnoozeResponse> {
        let payload = self.client.send(format!("{}?{:?}", "dnd.endSnooze", &self.params))?;
        payload.to_type::<DndEndSnoozeResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn dnd_end_snooze<>(&mut self, ) -> EndSnoozeBuilder {
        EndSnoozeBuilder::default(self,
        
        )
    }
}