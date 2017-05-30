//! # files_shared_public_url
//!
//! This method enables public/external sharing for a file.  
//! See https://api.slack.com/methods/files.sharedPublicURL  
//!
//! ## Required arguments: 
//!  `file` (String)  
//!  File to share.  
//!  Example: F1234567890


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
pub struct SharedPublicURLOptions {
/// File to share.
    pub file: String,

}

impl<'a> SharedPublicURLOptions {

    /// Create a new instance of SharedPublicURLOptions 
    fn new<S: Into<String>>(file: S) -> SharedPublicURLOptions {
        SharedPublicURLOptions { 
        file: file.into(),
             ..Default::default()
        }
    }
}

impl From<SharedPublicURLOptions> for String {
    fn from(options: SharedPublicURLOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The SharedPublicURLBuilder provides a fluid interface to create
/// objects of type SharedPublicURLOptions
pub struct SharedPublicURLBuilder<'a> {
    client: &'a mut Client,
    params: SharedPublicURLOptions,
}

impl<'a> SharedPublicURLBuilder<'a> {

    /// Create a default SharedPublicURLBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, file: S) -> SharedPublicURLBuilder<'a> {
        SharedPublicURLBuilder {
            client: client,
            params: SharedPublicURLOptions::new(
            file,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a SharedPublicURLResponse
    pub fn send(&mut self) -> Result<FilesSharedPublicURLResponse> {
        let payload = self.client.send(format!("{}?{:?}", "files.sharedPublicURL", &self.params))?;
        payload.to_type::<FilesSharedPublicURLResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn files_shared_public_url<S: Into<String>>(&mut self, file: S) -> SharedPublicURLBuilder {
        SharedPublicURLBuilder::default(self,
        file.into(),
        )
    }
}