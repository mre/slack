//! # files_upload
//!
//! This method allows you to create or upload an existing file.  
//! See https://api.slack.com/methods/files.upload  
//!
//! ## Required arguments: 
//!  `filename` (String)  
//!  Filename of file.  
//!  Example: foo.txt
//!
//! ## Optional arguments: 
//!
//!  `channels` (String)  
//!  Comma-separated list of channel names or IDs where the file will be shared.  
//!  Example: C1234567890  
//!
//!  `content` (String)  
//!  File contents via a POST variable. If omitting this parameter, you must provide a file.  
//!  Example: ...  
//!
//!  `file` (String)  
//!  File contents via multipart/form-data. If omitting this parameter, you must submit content.  
//!  Example: ...  
//!
//!  `filetype` (String)  
//!  A file type identifier.  
//!  Example: php  
//!
//!  `initial_comment` (String)  
//!  Initial comment to add to file.  
//!  Example: Best!  
//!
//!  `title` (String)  
//!  Title of file.  
//!  Example: My File

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
pub struct UploadOptions {
/// Filename of file.
    pub filename: String,
/// Comma-separated list of channel names or IDs where the file will be shared.
    pub channels: Option<String>,
/// File contents via a POST variable. If omitting this parameter, you must provide a file.
    pub content: Option<String>,
/// File contents via multipart/form-data. If omitting this parameter, you must submit content.
    pub file: Option<String>,
/// A file type identifier.
    pub filetype: Option<String>,
/// Initial comment to add to file.
    pub initial_comment: Option<String>,
/// Title of file.
    pub title: Option<String>,
}

impl<'a> UploadOptions {

    /// Create a new instance of UploadOptions 
    fn new<S: Into<String>>(filename: S) -> UploadOptions {
        UploadOptions { 
        filename: filename.into(),
             ..Default::default()
        }
    }
}

impl From<UploadOptions> for String {
    fn from(options: UploadOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The UploadBuilder provides a fluid interface to create
/// objects of type UploadOptions
pub struct UploadBuilder<'a> {
    client: &'a mut Client,
    params: UploadOptions,
}

impl<'a> UploadBuilder<'a> {

    /// Create a default UploadBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, filename: S) -> UploadBuilder<'a> {
        UploadBuilder {
            client: client,
            params: UploadOptions::new(
            filename,
            ),
        }
    }
/// Comma-separated list of channel names or IDs where the file will be shared.
    pub fn channels<S: Into<String>>(&'a mut self, channels: S) -> &mut UploadBuilder {
        self.params.channels = Some(channels.into());
        self
    }
/// File contents via a POST variable. If omitting this parameter, you must provide a file.
    pub fn content<S: Into<String>>(&'a mut self, content: S) -> &mut UploadBuilder {
        self.params.content = Some(content.into());
        self
    }
/// File contents via multipart/form-data. If omitting this parameter, you must submit content.
    pub fn file<S: Into<String>>(&'a mut self, file: S) -> &mut UploadBuilder {
        self.params.file = Some(file.into());
        self
    }
/// A file type identifier.
    pub fn filetype<S: Into<String>>(&'a mut self, filetype: S) -> &mut UploadBuilder {
        self.params.filetype = Some(filetype.into());
        self
    }
/// Initial comment to add to file.
    pub fn initial_comment<S: Into<String>>(&'a mut self, initial_comment: S) -> &mut UploadBuilder {
        self.params.initial_comment = Some(initial_comment.into());
        self
    }
/// Title of file.
    pub fn title<S: Into<String>>(&'a mut self, title: S) -> &mut UploadBuilder {
        self.params.title = Some(title.into());
        self
    }

    /// Send the request to Slack and try to convert the response to a UploadResponse
    pub fn send(&mut self) -> Result<FilesUploadResponse> {
        let payload = self.client.send(format!("{}?{:?}", "files.upload", &self.params))?;
        payload.to_type::<FilesUploadResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn files_upload<S: Into<String>>(&mut self, filename: S) -> UploadBuilder {
        UploadBuilder::default(self,
        filename.into(),
        )
    }
}