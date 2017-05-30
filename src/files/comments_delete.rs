//! # files_comments_delete
//!
//! Delete an existing comment on a file. Only the original author of the comment or a Team Administrator may delete a file comment.  
//! See https://api.slack.com/methods/files.comments.delete  
//!
//! ## Required arguments: 
//!  `file` (String)  
//!  File to delete a comment from.  
//!  Example: F1234567890  
//!  `id` (String)  
//!  The comment to delete.  
//!  Example: Fc1234567890


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
pub struct CommentsDeleteOptions {
/// File to delete a comment from.
    pub file: String,
/// The comment to delete.
    pub id: String,

}

impl<'a> CommentsDeleteOptions {

    /// Create a new instance of CommentsDeleteOptions 
    fn new<S: Into<String>>(file: S, id: S) -> CommentsDeleteOptions {
        CommentsDeleteOptions { 
        file: file.into(),
        id: id.into(),
             ..Default::default()
        }
    }
}

impl From<CommentsDeleteOptions> for String {
    fn from(options: CommentsDeleteOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The CommentsDeleteBuilder provides a fluid interface to create
/// objects of type CommentsDeleteOptions
pub struct CommentsDeleteBuilder<'a> {
    client: &'a mut Client,
    params: CommentsDeleteOptions,
}

impl<'a> CommentsDeleteBuilder<'a> {

    /// Create a default CommentsDeleteBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, file: S, id: S) -> CommentsDeleteBuilder<'a> {
        CommentsDeleteBuilder {
            client: client,
            params: CommentsDeleteOptions::new(
            file,
            id,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a CommentsDeleteResponse
    pub fn send(&mut self) -> Result<FilesCommentsDeleteResponse> {
        let payload = self.client.send(format!("{}?{:?}", "files.comments.delete", &self.params))?;
        payload.to_type::<FilesCommentsDeleteResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn files_comments_delete<S: Into<String>>(&mut self, file: S, id: S) -> CommentsDeleteBuilder {
        CommentsDeleteBuilder::default(self,
        file.into(),
        id.into(),
        )
    }
}