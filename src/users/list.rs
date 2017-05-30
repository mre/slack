//! # users_list
//!
//! This method returns a list of all users in the team. This includes deleted/deactivated users.  
//! See https://api.slack.com/methods/users.list  
//!

//!
//! ## Optional arguments: 
//!
//!  `presence` (bool)  
//!  Whether to include presence data in the output.  
//!  Example: 1

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

/// Whether to include presence data in the output.
    pub presence: Option<bool>,
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
/// Whether to include presence data in the output.
    pub fn presence(&'a mut self, presence: bool) -> &mut ListBuilder {
        self.params.presence = Some(presence);
        self
    }

    /// Send the request to Slack and try to convert the response to a ListResponse
    pub fn send(&mut self) -> Result<UsersListResponse> {
        let payload = self.client.send(format!("{}?{:?}", "users.list", &self.params))?;
        payload.to_type::<UsersListResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn users_list<>(&mut self, ) -> ListBuilder {
        ListBuilder::default(self,
        
        )
    }
}