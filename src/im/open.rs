//! # im_open
//!
//! This method opens a direct message channel with another member of your Slack team.  
//! See https://api.slack.com/methods/im.open  
//!
//! ## Required arguments: 
//!  `user` (String)  
//!  User to open a direct message channel with.  
//!  Example: U1234567890
//!
//! ## Optional arguments: 
//!
//!  `return_im` (String)  
//!  Boolean, indicates you want the full IM channel definition in the response.  
//!  Example:

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
/// User to open a direct message channel with.
    pub user: String,
/// Boolean, indicates you want the full IM channel definition in the response.
    pub return_im: Option<String>,
}

impl<'a> OpenOptions {

    /// Create a new instance of OpenOptions 
    fn new<S: Into<String>>(user: S) -> OpenOptions {
        OpenOptions { 
        user: user.into(),
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
    pub fn default<S: Into<String>>(client: &'a mut Client, user: S) -> OpenBuilder<'a> {
        OpenBuilder {
            client: client,
            params: OpenOptions::new(
            user,
            ),
        }
    }
/// Boolean, indicates you want the full IM channel definition in the response.
    pub fn return_im<S: Into<String>>(&'a mut self, return_im: S) -> &mut OpenBuilder {
        self.params.return_im = Some(return_im.into());
        self
    }

    /// Send the request to Slack and try to convert the response to a OpenResponse
    pub fn send(&mut self) -> Result<ImOpenResponse> {
        let payload = self.client.send(format!("{}?{:?}", "im.open", &self.params))?;
        payload.to_type::<ImOpenResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn im_open<S: Into<String>>(&mut self, user: S) -> OpenBuilder {
        OpenBuilder::default(self,
        user.into(),
        )
    }
}