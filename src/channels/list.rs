//! # channels_list
//!
//! This method returns a list of all channels in the team. This includes channels the caller is in, channels they are not currently in, and archived channels but does not include private channels. The number of (non-deactivated) members in each channel is also returned.  
//! See https://api.slack.com/methods/channels.list  
//!

//!
//! ## Optional arguments: 
//!
//!  `exclude_archived` (bool)  
//!  Don't return archived channels.  
//!  Example: 1

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

/// Don't return archived channels.
    pub exclude_archived: Option<bool>,
}

impl<'a> ListOptions {

    /// Create a new instance of ListOptions 
    fn new<>() -> ListOptions {
        ListOptions { 
        
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
    pub fn default<>(client: &'a mut Client, ) -> ListBuilder<'a> {
        ListBuilder {
            client: client,
            params: ListOptions::new(
            
            ),
        }
    }
/// Don't return archived channels.
    pub fn exclude_archived(&'a mut self, exclude_archived: bool) -> &mut ListBuilder {
        self.params.exclude_archived = Some(exclude_archived);
        self
    }

    /// Send the request to Slack and try to convert the response to a ListResponse
    pub fn send(&mut self) -> Result<ChannelsListResponse> {
        let payload = self.client.send(format!("{}?{:?}", "channels.list", &self.params))?;
        payload.to_type::<ChannelsListResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn channels_list<>(&mut self, ) -> ListBuilder {
        ListBuilder::default(self,
        
        )
    }
}