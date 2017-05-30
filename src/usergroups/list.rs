//! # usergroups_list
//!
//! This method returns a list of all User Groups in the team. This can optionally include disabled User Groups.  
//! See https://api.slack.com/methods/usergroups.list  
//!

//!
//! ## Optional arguments: 
//!
//!  `include_count` (bool)  
//!  Include the number of users in each User Group.  
//!  Example: 1  
//!
//!  `include_disabled` (bool)  
//!  Include disabled User Groups.  
//!  Example: 1  
//!
//!  `include_users` (bool)  
//!  Include the list of users for each User Group.  
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

/// Include the number of users in each User Group.
    pub include_count: Option<bool>,
/// Include disabled User Groups.
    pub include_disabled: Option<bool>,
/// Include the list of users for each User Group.
    pub include_users: Option<bool>,
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
/// Include the number of users in each User Group.
    pub fn include_count(&'a mut self, include_count: bool) -> &mut ListBuilder {
        self.params.include_count = Some(include_count);
        self
    }
/// Include disabled User Groups.
    pub fn include_disabled(&'a mut self, include_disabled: bool) -> &mut ListBuilder {
        self.params.include_disabled = Some(include_disabled);
        self
    }
/// Include the list of users for each User Group.
    pub fn include_users(&'a mut self, include_users: bool) -> &mut ListBuilder {
        self.params.include_users = Some(include_users);
        self
    }

    /// Send the request to Slack and try to convert the response to a ListResponse
    pub fn send(&mut self) -> Result<UsergroupsListResponse> {
        let payload = self.client.send(format!("{}?{:?}", "usergroups.list", &self.params))?;
        payload.to_type::<UsergroupsListResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn usergroups_list<>(&mut self, ) -> ListBuilder {
        ListBuilder::default(self,
        
        )
    }
}