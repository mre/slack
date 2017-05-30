//! # search_messages
//!
//! This method returns messages matching a search query.  
//! See https://api.slack.com/methods/search.messages  
//!
//! ## Required arguments: 
//!  `query` (String)  
//!  Search query. May contains booleans, etc.  
//!  Example: pickleface
//!
//! ## Optional arguments: 
//!
//!  `highlight` (bool)  
//!  Pass a value of 1 to enable query highlight markers (see below).  
//!  Example: 1  
//!
//!  `sort` (String)  
//!  Return matches sorted by either score or timestamp.  
//!  Example: timestamp  
//!
//!  `sort_dir` (String)  
//!  Change sort direction to ascending (asc) or descending (desc).  
//!  Example: asc

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
pub struct MessagesOptions {
/// Search query. May contains booleans, etc.
    pub query: String,
/// Pass a value of 1 to enable query highlight markers (see below).
    pub highlight: Option<bool>,
/// Return matches sorted by either score or timestamp.
    pub sort: Option<String>,
/// Change sort direction to ascending (asc) or descending (desc).
    pub sort_dir: Option<String>,
}

impl<'a> MessagesOptions {

    /// Create a new instance of MessagesOptions 
    fn new<S: Into<String>>(query: S) -> MessagesOptions {
        MessagesOptions { 
        query: query.into(),
             ..Default::default()
        }
    }
}

impl From<MessagesOptions> for String {
    fn from(options: MessagesOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The MessagesBuilder provides a fluid interface to create
/// objects of type MessagesOptions
pub struct MessagesBuilder<'a> {
    client: &'a mut Client,
    params: MessagesOptions,
}

impl<'a> MessagesBuilder<'a> {

    /// Create a default MessagesBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, query: S) -> MessagesBuilder<'a> {
        MessagesBuilder {
            client: client,
            params: MessagesOptions::new(
            query,
            ),
        }
    }
/// Pass a value of 1 to enable query highlight markers (see below).
    pub fn highlight(&'a mut self, highlight: bool) -> &mut MessagesBuilder {
        self.params.highlight = Some(highlight);
        self
    }
/// Return matches sorted by either score or timestamp.
    pub fn sort<S: Into<String>>(&'a mut self, sort: S) -> &mut MessagesBuilder {
        self.params.sort = Some(sort.into());
        self
    }
/// Change sort direction to ascending (asc) or descending (desc).
    pub fn sort_dir<S: Into<String>>(&'a mut self, sort_dir: S) -> &mut MessagesBuilder {
        self.params.sort_dir = Some(sort_dir.into());
        self
    }

    /// Send the request to Slack and try to convert the response to a MessagesResponse
    pub fn send(&mut self) -> Result<SearchMessagesResponse> {
        let payload = self.client.send(format!("{}?{:?}", "search.messages", &self.params))?;
        payload.to_type::<SearchMessagesResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn search_messages<S: Into<String>>(&mut self, query: S) -> MessagesBuilder {
        MessagesBuilder::default(self,
        query.into(),
        )
    }
}