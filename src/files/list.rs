//! # files_list
//!
//! This method returns a list of files within the team. It can be filtered and sliced in various ways.  
//! See https://api.slack.com/methods/files.list  
//!

//!
//! ## Optional arguments: 
//!
//!  `channel` (String)  
//!  Filter files appearing in a specific channel, indicated by its ID.  
//!  Example: C1234567890  
//!
//!  `ts_from` (i64)  
//!  Filter files created after this timestamp (inclusive).  
//!  Example: 123456789  
//!
//!  `ts_to` (i64)  
//!  Filter files created before this timestamp (inclusive).  
//!  Example: 123456789  
//!
//!  `types` (String)  
//!  Filter files by type:. . all - All files. spaces - Posts. snippets - Snippets. images - Image files. gdocs - Google docs. zips - Zip files. pdfs - PDF files. . . You can pass multiple values in the types argument, like types=spaces,snippets.The default value is all, which does not filter the list.. .  
//!  Example: images  
//!
//!  `user` (String)  
//!  Filter files created by a single user.  
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

/// Filter files appearing in a specific channel, indicated by its ID.
    pub channel: Option<String>,
/// Filter files created after this timestamp (inclusive).
    pub ts_from: Option<i64>,
/// Filter files created before this timestamp (inclusive).
    pub ts_to: Option<i64>,
/// Filter files by type:. . all - All files. spaces - Posts. snippets - Snippets. images - Image files. gdocs - Google docs. zips - Zip files. pdfs - PDF files. . . You can pass multiple values in the types argument, like types=spaces,snippets.The default value is all, which does not filter the list.. .
    pub types: Option<String>,
/// Filter files created by a single user.
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
/// Filter files appearing in a specific channel, indicated by its ID.
    pub fn channel<S: Into<String>>(&'a mut self, channel: S) -> &mut ListBuilder {
        self.params.channel = Some(channel.into());
        self
    }
/// Filter files created after this timestamp (inclusive).
    pub fn ts_from(&'a mut self, ts_from: i64) -> &mut ListBuilder {
        self.params.ts_from = Some(ts_from);
        self
    }
/// Filter files created before this timestamp (inclusive).
    pub fn ts_to(&'a mut self, ts_to: i64) -> &mut ListBuilder {
        self.params.ts_to = Some(ts_to);
        self
    }
/// Filter files by type:. . all - All files. spaces - Posts. snippets - Snippets. images - Image files. gdocs - Google docs. zips - Zip files. pdfs - PDF files. . . You can pass multiple values in the types argument, like types=spaces,snippets.The default value is all, which does not filter the list.. .
    pub fn types<S: Into<String>>(&'a mut self, types: S) -> &mut ListBuilder {
        self.params.types = Some(types.into());
        self
    }
/// Filter files created by a single user.
    pub fn user<S: Into<String>>(&'a mut self, user: S) -> &mut ListBuilder {
        self.params.user = Some(user.into());
        self
    }

    /// Send the request to Slack and try to convert the response to a ListResponse
    pub fn send(&mut self) -> Result<FilesListResponse> {
        let payload = self.client.send(format!("{}?{:?}", "files.list", &self.params))?;
        payload.to_type::<FilesListResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn files_list<>(&mut self, ) -> ListBuilder {
        ListBuilder::default(self,
        
        )
    }
}