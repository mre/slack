//! # bots_info
//!
//! This method returns information about a bot user.  
//! See https://api.slack.com/methods/bots.info  
//!

//!
//! ## Optional arguments: 
//!
//!  `bot` (String)  
//!  Bot user to get info on.  
//!  Example: B12345678

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

/// Bot user to get info on.
    pub bot: Option<String>,
}

impl<'a> InfoOptions {

    /// Create a new instance of InfoOptions 
    fn new<>() -> InfoOptions {
        InfoOptions { 
        
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
    pub fn default<>(client: &'a mut Client, ) -> InfoBuilder<'a> {
        InfoBuilder {
            client: client,
            params: InfoOptions::new(
            
            ),
        }
    }
/// Bot user to get info on.
    pub fn bot<S: Into<String>>(&'a mut self, bot: S) -> &mut InfoBuilder {
        self.params.bot = Some(bot.into());
        self
    }

    /// Send the request to Slack and try to convert the response to a InfoResponse
    pub fn send(&mut self) -> Result<BotsInfoResponse> {
        let payload = self.client.send(format!("{}?{:?}", "bots.info", &self.params))?;
        payload.to_type::<BotsInfoResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn bots_info<>(&mut self, ) -> InfoBuilder {
        InfoBuilder::default(self,
        
        )
    }
}