//! # dnd_end_dnd
//!
//! Ends the user's currently scheduled Do Not Disturb session immediately.  
//! See https://api.slack.com/methods/dnd.endDnd  
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
pub struct EndDndOptions {


}

impl<'a> EndDndOptions {

    /// Create a new instance of EndDndOptions 
    fn new<>() -> EndDndOptions {
        EndDndOptions { 
        
             ..Default::default()
        }
    }
}

impl From<EndDndOptions> for String {
    fn from(options: EndDndOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The EndDndBuilder provides a fluid interface to create
/// objects of type EndDndOptions
pub struct EndDndBuilder<'a> {
    client: &'a mut Client,
    params: EndDndOptions,
}

impl<'a> EndDndBuilder<'a> {

    /// Create a default EndDndBuilder object
    pub fn default<>(client: &'a mut Client, ) -> EndDndBuilder<'a> {
        EndDndBuilder {
            client: client,
            params: EndDndOptions::new(
            
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a EndDndResponse
    pub fn send(&mut self) -> Result<DndEndDndResponse> {
        let payload = self.client.send(format!("{}?{:?}", "dnd.endDnd", &self.params))?;
        payload.to_type::<DndEndDndResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn dnd_end_dnd<>(&mut self, ) -> EndDndBuilder {
        EndDndBuilder::default(self,
        
        )
    }
}