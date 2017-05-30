//! # pins_list
//!
//! This method lists the items pinned to a channel.  
//! See https://api.slack.com/methods/pins.list  
//!
//! ## Required arguments: 
//!  `channel` (String)  
//!  Channel to get pinned items for.  
//!  Example: C1234567890


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
pub struct ListOptions {
/// Channel to get pinned items for.
    pub channel: String,

}

impl<'a> ListOptions {

    /// Create a new instance of ListOptions 
    fn new<S: Into<String>>(channel: S) -> ListOptions {
        ListOptions { 
        channel: channel.into(),
             ..Default::default()
        }
    }
}

impl From<ListOptions> for String {
    fn from(options: ListOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The ListBuilder provides a fluid interface to create
/// objects of type ListOptions
pub struct ListBuilder<'a> {
    client: &'a mut Client,
    params: ListOptions,
}

impl<'a> ListBuilder<'a> {

    /// Create a default ListBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, channel: S) -> ListBuilder<'a> {
        ListBuilder {
            client: client,
            params: ListOptions::new(
            channel,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a ListResponse
    pub fn send(&mut self) -> Result<PinsListResponse> {
        let payload = self.client.send(format!("{}?{:?}", "pins.list", &self.params))?;
        payload.to_type::<PinsListResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn pins_list<S: Into<String>>(&mut self, channel: S) -> ListBuilder {
        ListBuilder::default(self,
        channel.into(),
        )
    }
}