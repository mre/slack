//! # mpim_open
//!
//! This method opens a multiparty direct message.  
//! See https://api.slack.com/methods/mpim.open  
//!
//! ## Required arguments: 
//!  `users` (String)  
//!  Comma separated lists of users.  The ordering of the users is preserved whenever a MPIM group is returned.  
//!  Example: U1234567890,U2345678901,U3456789012


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
pub struct OpenOptions {
/// Comma separated lists of users.  The ordering of the users is preserved whenever a MPIM group is returned.
    pub users: String,

}

impl<'a> OpenOptions {

    /// Create a new instance of OpenOptions 
    fn new<S: Into<String>>(users: S) -> OpenOptions {
        OpenOptions { 
        users: users.into(),
             ..Default::default()
        }
    }
}

impl From<OpenOptions> for String {
    fn from(options: OpenOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The OpenBuilder provides a fluid interface to create
/// objects of type OpenOptions
pub struct OpenBuilder<'a> {
    client: &'a mut Client,
    params: OpenOptions,
}

impl<'a> OpenBuilder<'a> {

    /// Create a default OpenBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, users: S) -> OpenBuilder<'a> {
        OpenBuilder {
            client: client,
            params: OpenOptions::new(
            users,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a OpenResponse
    pub fn send(&mut self) -> Result<MpimOpenResponse> {
        let payload = self.client.send(format!("{}?{:?}", "mpim.open", &self.params))?;
        payload.to_type::<MpimOpenResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn mpim_open<S: Into<String>>(&mut self, users: S) -> OpenBuilder {
        OpenBuilder::default(self,
        users.into(),
        )
    }
}