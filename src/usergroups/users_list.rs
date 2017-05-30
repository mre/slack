//! # usergroups_users_list
//!
//! This method returns a list of all users within a User Group.  
//! See https://api.slack.com/methods/usergroups.users.list  
//!
//! ## Required arguments: 
//!  `usergroup` (String)  
//!  The encoded ID of the User Group to update.  
//!  Example: S0604QSJC
//!
//! ## Optional arguments: 
//!
//!  `include_disabled` (bool)  
//!  Allow results that involve disabled User Groups.  
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
pub struct UsersListOptions {
/// The encoded ID of the User Group to update.
    pub usergroup: String,
/// Allow results that involve disabled User Groups.
    pub include_disabled: Option<bool>,
}

impl<'a> UsersListOptions {

    /// Create a new instance of UsersListOptions 
    fn new<S: Into<String>>(usergroup: S) -> UsersListOptions {
        UsersListOptions { 
        usergroup: usergroup.into(),
             ..Default::default()
        }
    }
}

impl From<UsersListOptions> for String {
    fn from(options: UsersListOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The UsersListBuilder provides a fluid interface to create
/// objects of type UsersListOptions
pub struct UsersListBuilder<'a> {
    client: &'a mut Client,
    params: UsersListOptions,
}

impl<'a> UsersListBuilder<'a> {

    /// Create a default UsersListBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, usergroup: S) -> UsersListBuilder<'a> {
        UsersListBuilder {
            client: client,
            params: UsersListOptions::new(
            usergroup,
            ),
        }
    }
/// Allow results that involve disabled User Groups.
    pub fn include_disabled(&'a mut self, include_disabled: bool) -> &mut UsersListBuilder {
        self.params.include_disabled = Some(include_disabled);
        self
    }

    /// Send the request to Slack and try to convert the response to a UsersListResponse
    pub fn send(&mut self) -> Result<UsergroupsUsersListResponse> {
        let payload = self.client.send(format!("{}?{:?}", "usergroups.users.list", &self.params))?;
        payload.to_type::<UsergroupsUsersListResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn usergroups_users_list<S: Into<String>>(&mut self, usergroup: S) -> UsersListBuilder {
        UsersListBuilder::default(self,
        usergroup.into(),
        )
    }
}