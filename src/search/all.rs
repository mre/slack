//! # search_all
//!
//! This method allows users and applications to search both messages and files in a single call.  
//! See https://api.slack.com/methods/search.all  
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
pub struct AllOptions {
/// Search query. May contains booleans, etc.
    pub query: String,
/// Pass a value of 1 to enable query highlight markers (see below).
    pub highlight: Option<bool>,
/// Return matches sorted by either score or timestamp.
    pub sort: Option<String>,
/// Change sort direction to ascending (asc) or descending (desc).
    pub sort_dir: Option<String>,
}

impl<'a> AllOptions {

    /// Create a new instance of AllOptions 
    fn new<S: Into<String>>(query: S) -> AllOptions {
        AllOptions { 
        query: query.into(),
             ..Default::default()
        }
    }
}

impl From<AllOptions> for String {
    fn from(options: AllOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The AllBuilder provides a fluid interface to create
/// objects of type AllOptions
pub struct AllBuilder<'a> {
    client: &'a mut Client,
    params: AllOptions,
}

impl<'a> AllBuilder<'a> {

    /// Create a default AllBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, query: S) -> AllBuilder<'a> {
        AllBuilder {
            client: client,
            params: AllOptions::new(
            query,
            ),
        }
    }
/// Pass a value of 1 to enable query highlight markers (see below).
    pub fn highlight(&'a mut self, highlight: bool) -> &mut AllBuilder {
        self.params.highlight = Some(highlight);
        self
    }
/// Return matches sorted by either score or timestamp.
    pub fn sort<S: Into<String>>(&'a mut self, sort: S) -> &mut AllBuilder {
        self.params.sort = Some(sort.into());
        self
    }
/// Change sort direction to ascending (asc) or descending (desc).
    pub fn sort_dir<S: Into<String>>(&'a mut self, sort_dir: S) -> &mut AllBuilder {
        self.params.sort_dir = Some(sort_dir.into());
        self
    }

    /// Send the request to Slack and try to convert the response to a AllResponse
    pub fn send(&mut self) -> Result<SearchAllResponse> {
        let payload = self.client.send(format!("{}?{:?}", "search.all", &self.params))?;
        payload.to_type::<SearchAllResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn search_all<S: Into<String>>(&mut self, query: S) -> AllBuilder {
        AllBuilder::default(self,
        query.into(),
        )
    }
}