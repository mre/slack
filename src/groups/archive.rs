//! # groups_archive
//!
//! This method archives a private channel.  
//! See https://api.slack.com/methods/groups.archive  
//!
//! ## Required arguments: 
//!  `channel` (String)  
//!  Private channel to archive.  
//!  Example: G1234567890


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
pub struct ArchiveOptions {
/// Private channel to archive.
    pub channel: String,

}

impl<'a> ArchiveOptions {

    /// Create a new instance of ArchiveOptions 
    fn new<S: Into<String>>(channel: S) -> ArchiveOptions {
        ArchiveOptions { 
        channel: channel.into(),
             ..Default::default()
        }
    }
}

impl From<ArchiveOptions> for String {
    fn from(options: ArchiveOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The ArchiveBuilder provides a fluid interface to create
/// objects of type ArchiveOptions
pub struct ArchiveBuilder<'a> {
    client: &'a mut Client,
    params: ArchiveOptions,
}

impl<'a> ArchiveBuilder<'a> {

    /// Create a default ArchiveBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, channel: S) -> ArchiveBuilder<'a> {
        ArchiveBuilder {
            client: client,
            params: ArchiveOptions::new(
            channel,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a ArchiveResponse
    pub fn send(&mut self) -> Result<GroupsArchiveResponse> {
        let payload = self.client.send(format!("{}?{:?}", "groups.archive", &self.params))?;
        payload.to_type::<GroupsArchiveResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn groups_archive<S: Into<String>>(&mut self, channel: S) -> ArchiveBuilder {
        ArchiveBuilder::default(self,
        channel.into(),
        )
    }
}