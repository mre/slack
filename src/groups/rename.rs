//! # groups_rename
//!
//! This method renames a private channel.  
//! See https://api.slack.com/methods/groups.rename  
//!
//! ## Required arguments: 
//!  `channel` (String)  
//!  Private channel to rename.  
//!  Example: C1234567890  
//!  `name` (String)  
//!  New name for private channel.  
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
pub struct RenameOptions {
/// Private channel to rename.
    pub channel: String,
/// New name for private channel.
    pub name: String,

}

impl<'a> RenameOptions {

    /// Create a new instance of RenameOptions 
    fn new<S: Into<String>>(channel: S, name: S) -> RenameOptions {
        RenameOptions { 
        channel: channel.into(),
        name: name.into(),
             ..Default::default()
        }
    }
}

impl From<RenameOptions> for String {
    fn from(options: RenameOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The RenameBuilder provides a fluid interface to create
/// objects of type RenameOptions
pub struct RenameBuilder<'a> {
    client: &'a mut Client,
    params: RenameOptions,
}

impl<'a> RenameBuilder<'a> {

    /// Create a default RenameBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, channel: S, name: S) -> RenameBuilder<'a> {
        RenameBuilder {
            client: client,
            params: RenameOptions::new(
            channel,
            name,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a RenameResponse
    pub fn send(&mut self) -> Result<GroupsRenameResponse> {
        let payload = self.client.send(format!("{}?{:?}", "groups.rename", &self.params))?;
        payload.to_type::<GroupsRenameResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn groups_rename<S: Into<String>>(&mut self, channel: S, name: S) -> RenameBuilder {
        RenameBuilder::default(self,
        channel.into(),
        name.into(),
        )
    }
}