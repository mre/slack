//! # reactions_list
//!
//! This method returns a list of all items (file, file comment, channel message, group message, or direct message) reacted to by a user.  
//! See https://api.slack.com/methods/reactions.list  
//!

//!
//! ## Optional arguments: 
//!
//!  `full` (String)  
//!  If true always return the complete reaction list.  
//!  Example:    
//!
//!  `user` (String)  
//!  Show reactions made by this user. Defaults to the authed user.  
//!  Example: U1234567890

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
pub struct ListOptions {

/// If true always return the complete reaction list.
    pub full: Option<String>,
/// Show reactions made by this user. Defaults to the authed user.
    pub user: Option<String>,
}

impl<'a> ListOptions {

    /// Create a new instance of ListOptions 
    fn new<>() -> ListOptions {
        ListOptions { 
        
             ..Default::default()
        }
    }
}

impl From<ListOptions> for String {
    fn from(options: ListOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The ListBuilder provides a fluid interface to create
/// objects of type ListOptions
pub struct ListBuilder<'a> {
    client: &'a mut Client,
    params: ListOptions,
}

impl<'a> ListBuilder<'a> {

    /// Create a default ListBuilder object
    pub fn default<>(client: &'a mut Client, ) -> ListBuilder<'a> {
        ListBuilder {
            client: client,
            params: ListOptions::new(
            
            ),
        }
    }
/// If true always return the complete reaction list.
    pub fn full<S: Into<String>>(&'a mut self, full: S) -> &mut ListBuilder {
        self.params.full = Some(full.into());
        self
    }
/// Show reactions made by this user. Defaults to the authed user.
    pub fn user<S: Into<String>>(&'a mut self, user: S) -> &mut ListBuilder {
        self.params.user = Some(user.into());
        self
    }

    /// Send the request to Slack and try to convert the response to a ListResponse
    pub fn send(&mut self) -> Result<ReactionsListResponse> {
        let payload = self.client.send(format!("{}?{:?}", "reactions.list", &self.params))?;
        payload.to_type::<ReactionsListResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn reactions_list<>(&mut self, ) -> ListBuilder {
        ListBuilder::default(self,
        
        )
    }
}