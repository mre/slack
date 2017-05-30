//! # files_comments_edit
//!
//! Edit an existing comment on a file. Only the user who created a comment may make edits. Teams may configure a limited time window during which file comment edits are allowed.  
//! See https://api.slack.com/methods/files.comments.edit  
//!
//! ## Required arguments: 
//!  `comment` (String)  
//!  Text of the comment to edit.  
//!  Example: Everyone should take a moment to read this file, seriously.  
//!  `file` (String)  
//!  File containing the comment to edit.  
//!  Example: F1234567890  
//!  `id` (String)  
//!  The comment to edit.  
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
pub struct CommentsEditOptions {
/// Text of the comment to edit.
    pub comment: String,
/// File containing the comment to edit.
    pub file: String,
/// The comment to edit.
    pub id: String,

}

impl<'a> CommentsEditOptions {

    /// Create a new instance of CommentsEditOptions 
    fn new<S: Into<String>>(comment: S, file: S, id: S) -> CommentsEditOptions {
        CommentsEditOptions { 
        comment: comment.into(),
        file: file.into(),
        id: id.into(),
             ..Default::default()
        }
    }
}

impl From<CommentsEditOptions> for String {
    fn from(options: CommentsEditOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The CommentsEditBuilder provides a fluid interface to create
/// objects of type CommentsEditOptions
pub struct CommentsEditBuilder<'a> {
    client: &'a mut Client,
    params: CommentsEditOptions,
}

impl<'a> CommentsEditBuilder<'a> {

    /// Create a default CommentsEditBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, comment: S, file: S, id: S) -> CommentsEditBuilder<'a> {
        CommentsEditBuilder {
            client: client,
            params: CommentsEditOptions::new(
            comment,
            file,
            id,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a CommentsEditResponse
    pub fn send(&mut self) -> Result<FilesCommentsEditResponse> {
        let payload = self.client.send(format!("{}?{:?}", "files.comments.edit", &self.params))?;
        payload.to_type::<FilesCommentsEditResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn files_comments_edit<S: Into<String>>(&mut self, comment: S, file: S, id: S) -> CommentsEditBuilder {
        CommentsEditBuilder::default(self,
        comment.into(),
        file.into(),
        id.into(),
        )
    }
}