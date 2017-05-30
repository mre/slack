//! # files_revoke_public_url
//!
//! This method disables public/external sharing for a file.  
//! See https://api.slack.com/methods/files.revokePublicURL  
//!
//! ## Required arguments: 
//!  `file` (String)  
//!  File to revoke.  
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
pub struct RevokePublicURLOptions {
/// File to revoke.
    pub file: String,

}

impl<'a> RevokePublicURLOptions {

    /// Create a new instance of RevokePublicURLOptions 
    fn new<S: Into<String>>(file: S) -> RevokePublicURLOptions {
        RevokePublicURLOptions { 
        file: file.into(),
             ..Default::default()
        }
    }
}

impl From<RevokePublicURLOptions> for String {
    fn from(options: RevokePublicURLOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The RevokePublicURLBuilder provides a fluid interface to create
/// objects of type RevokePublicURLOptions
pub struct RevokePublicURLBuilder<'a> {
    client: &'a mut Client,
    params: RevokePublicURLOptions,
}

impl<'a> RevokePublicURLBuilder<'a> {

    /// Create a default RevokePublicURLBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, file: S) -> RevokePublicURLBuilder<'a> {
        RevokePublicURLBuilder {
            client: client,
            params: RevokePublicURLOptions::new(
            file,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a RevokePublicURLResponse
    pub fn send(&mut self) -> Result<FilesRevokePublicURLResponse> {
        let payload = self.client.send(format!("{}?{:?}", "files.revokePublicURL", &self.params))?;
        payload.to_type::<FilesRevokePublicURLResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn files_revoke_public_url<S: Into<String>>(&mut self, file: S) -> RevokePublicURLBuilder {
        RevokePublicURLBuilder::default(self,
        file.into(),
        )
    }
}