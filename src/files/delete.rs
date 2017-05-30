//! # files_delete
//!
//! This method deletes a file from your team.  
//! See https://api.slack.com/methods/files.delete  
//!
//! ## Required arguments: 
//!  `file` (String)  
//!  ID of file to delete.  
//!  Example:


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
pub struct DeleteOptions {
/// ID of file to delete.
    pub file: String,

}

impl<'a> DeleteOptions {

    /// Create a new instance of DeleteOptions 
    fn new<S: Into<String>>(file: S) -> DeleteOptions {
        DeleteOptions { 
        file: file.into(),
             ..Default::default()
        }
    }
}

impl From<DeleteOptions> for String {
    fn from(options: DeleteOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The DeleteBuilder provides a fluid interface to create
/// objects of type DeleteOptions
pub struct DeleteBuilder<'a> {
    client: &'a mut Client,
    params: DeleteOptions,
}

impl<'a> DeleteBuilder<'a> {

    /// Create a default DeleteBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, file: S) -> DeleteBuilder<'a> {
        DeleteBuilder {
            client: client,
            params: DeleteOptions::new(
            file,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a DeleteResponse
    pub fn send(&mut self) -> Result<FilesDeleteResponse> {
        let payload = self.client.send(format!("{}?{:?}", "files.delete", &self.params))?;
        payload.to_type::<FilesDeleteResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn files_delete<S: Into<String>>(&mut self, file: S) -> DeleteBuilder {
        DeleteBuilder::default(self,
        file.into(),
        )
    }
}