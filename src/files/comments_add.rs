//! # files_comments_add
//!
//! Add a comment to an existing file.  
//! See https://api.slack.com/methods/files.comments.add  
//!
//! ## Required arguments: 
//!  `comment` (String)  
//!  Text of the comment to add.  
//!  Example: Everyone should take a moment to read this file.  
//!  `file` (String)  
//!  File to add a comment to.  
//!  Example: F1234467890
//!
//! ## Optional arguments: 
//!
//!  `channel` (String)  
//!  Channel id (encoded) of which location to associate with the new comment.  
//!  Example: C1234467890

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
pub struct CommentsAddOptions {
/// Text of the comment to add.
    pub comment: String,
/// File to add a comment to.
    pub file: String,
/// Channel id (encoded) of which location to associate with the new comment.
    pub channel: Option<String>,
}

impl<'a> CommentsAddOptions {

    /// Create a new instance of CommentsAddOptions 
    fn new<S: Into<String>>(comment: S, file: S) -> CommentsAddOptions {
        CommentsAddOptions { 
        comment: comment.into(),
        file: file.into(),
             ..Default::default()
        }
    }
}

impl From<CommentsAddOptions> for String {
    fn from(options: CommentsAddOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The CommentsAddBuilder provides a fluid interface to create
/// objects of type CommentsAddOptions
pub struct CommentsAddBuilder<'a> {
    client: &'a mut Client,
    params: CommentsAddOptions,
}

impl<'a> CommentsAddBuilder<'a> {

    /// Create a default CommentsAddBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, comment: S, file: S) -> CommentsAddBuilder<'a> {
        CommentsAddBuilder {
            client: client,
            params: CommentsAddOptions::new(
            comment,
            file,
            ),
        }
    }
/// Channel id (encoded) of which location to associate with the new comment.
    pub fn channel<S: Into<String>>(&'a mut self, channel: S) -> &mut CommentsAddBuilder {
        self.params.channel = Some(channel.into());
        self
    }

    /// Send the request to Slack and try to convert the response to a CommentsAddResponse
    pub fn send(&mut self) -> Result<FilesCommentsAddResponse> {
        let payload = self.client.send(format!("{}?{:?}", "files.comments.add", &self.params))?;
        payload.to_type::<FilesCommentsAddResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn files_comments_add<S: Into<String>>(&mut self, comment: S, file: S) -> CommentsAddBuilder {
        CommentsAddBuilder::default(self,
        comment.into(),
        file.into(),
        )
    }
}