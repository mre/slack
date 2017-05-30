//! # usergroups_users_update
//!
//! This method updates the list of users that belong to a User Group. This method replaces all users in a User Group with the list of users provided in the users parameter.  
//! See https://api.slack.com/methods/usergroups.users.update  
//!
//! ## Required arguments: 
//!  `usergroup` (String)  
//!  The encoded ID of the User Group to update.  
//!  Example: S0604QSJC  
//!  `users` (String)  
//!  A comma separated string of encoded user IDs that represent the entire list of users for the User Group.  
//!  Example: U060R4BJ4,U060RNRCZ
//!
//! ## Optional arguments: 
//!
//!  `include_count` (bool)  
//!  Include the number of users in the User Group.  
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
pub struct UsersUpdateOptions {
/// The encoded ID of the User Group to update.
    pub usergroup: String,
/// A comma separated string of encoded user IDs that represent the entire list of users for the User Group.
    pub users: String,
/// Include the number of users in the User Group.
    pub include_count: Option<bool>,
}

impl<'a> UsersUpdateOptions {

    /// Create a new instance of UsersUpdateOptions 
    fn new<S: Into<String>>(usergroup: S, users: S) -> UsersUpdateOptions {
        UsersUpdateOptions { 
        usergroup: usergroup.into(),
        users: users.into(),
             ..Default::default()
        }
    }
}

impl From<UsersUpdateOptions> for String {
    fn from(options: UsersUpdateOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The UsersUpdateBuilder provides a fluid interface to create
/// objects of type UsersUpdateOptions
pub struct UsersUpdateBuilder<'a> {
    client: &'a mut Client,
    params: UsersUpdateOptions,
}

impl<'a> UsersUpdateBuilder<'a> {

    /// Create a default UsersUpdateBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, usergroup: S, users: S) -> UsersUpdateBuilder<'a> {
        UsersUpdateBuilder {
            client: client,
            params: UsersUpdateOptions::new(
            usergroup,
            users,
            ),
        }
    }
/// Include the number of users in the User Group.
    pub fn include_count(&'a mut self, include_count: bool) -> &mut UsersUpdateBuilder {
        self.params.include_count = Some(include_count);
        self
    }

    /// Send the request to Slack and try to convert the response to a UsersUpdateResponse
    pub fn send(&mut self) -> Result<UsergroupsUsersUpdateResponse> {
        let payload = self.client.send(format!("{}?{:?}", "usergroups.users.update", &self.params))?;
        payload.to_type::<UsergroupsUsersUpdateResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn usergroups_users_update<S: Into<String>>(&mut self, usergroup: S, users: S) -> UsersUpdateBuilder {
        UsersUpdateBuilder::default(self,
        usergroup.into(),
        users.into(),
        )
    }
}