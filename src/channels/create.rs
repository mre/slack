//! # channels_create
//!
//! This method is used to create a channel.  
//! See https://api.slack.com/methods/channels.create  
//!
//! ## Required arguments: 
//!  `name` (String)  
//!  Name of channel to create.  
//!  Example: mychannel


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
pub struct CreateOptions {
/// Name of channel to create.
    pub name: String,

}

impl<'a> CreateOptions {

    /// Create a new instance of CreateOptions 
    fn new<S: Into<String>>(name: S) -> CreateOptions {
        CreateOptions { 
        name: name.into(),
             ..Default::default()
        }
    }
}

impl From<CreateOptions> for String {
    fn from(options: CreateOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The CreateBuilder provides a fluid interface to create
/// objects of type CreateOptions
pub struct CreateBuilder<'a> {
    client: &'a mut Client,
    params: CreateOptions,
}

impl<'a> CreateBuilder<'a> {

    /// Create a default CreateBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, name: S) -> CreateBuilder<'a> {
        CreateBuilder {
            client: client,
            params: CreateOptions::new(
            name,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a CreateResponse
    pub fn send(&mut self) -> Result<ChannelsCreateResponse> {
        let payload = self.client.send(format!("{}?{:?}", "channels.create", &self.params))?;
        payload.to_type::<ChannelsCreateResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn channels_create<S: Into<String>>(&mut self, name: S) -> CreateBuilder {
        CreateBuilder::default(self,
        name.into(),
        )
    }
}