//! # oauth_access
//!
//! This method allows you to exchange a temporary OAuth code for an API access token.  
//! This is used as part of the OAuth authentication flow.  
//! See https://api.slack.com/methods/oauth.access  
//!
//! ## Required arguments: 
//!  `client_id` (String)  
//!  Issued when you created your application.  
//!  Example: 4b39e9-752c4  
//!  `client_secret` (String)  
//!  Issued when you created your application.  
//!  Example: 33fea0113f5b1  
//!  `code` (String)  
//!  The code param returned via the OAuth callback.  
//!  Example: ccdaa72ad
//!
//! ## Optional arguments: 
//!
//!  `redirect_uri` (String)  
//!  This must match the originally submitted URI (if one was sent).  
//!  Example: http://example.com

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
pub struct AccessOptions {
/// Issued when you created your application.
    pub client_id: String,
/// Issued when you created your application.
    pub client_secret: String,
/// The code param returned via the OAuth callback.
    pub code: String,
/// This must match the originally submitted URI (if one was sent).
    pub redirect_uri: Option<String>,
}

impl<'a> AccessOptions {

    /// Create a new instance of AccessOptions 
    fn new<S: Into<String>>(client_id: S, client_secret: S, code: S) -> AccessOptions {
        AccessOptions { 
        client_id: client_id.into(),
        client_secret: client_secret.into(),
        code: code.into(),
             ..Default::default()
        }
    }
}

impl From<AccessOptions> for String {
    fn from(options: AccessOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The AccessBuilder provides a fluid interface to create
/// objects of type AccessOptions
pub struct AccessBuilder<'a> {
    client: &'a mut Client,
    params: AccessOptions,
}

impl<'a> AccessBuilder<'a> {

    /// Create a default AccessBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, client_id: S, client_secret: S, code: S) -> AccessBuilder<'a> {
        AccessBuilder {
            client: client,
            params: AccessOptions::new(
            client_id,
            client_secret,
            code,
            ),
        }
    }
/// This must match the originally submitted URI (if one was sent).
    pub fn redirect_uri<S: Into<String>>(&'a mut self, redirect_uri: S) -> &mut AccessBuilder {
        self.params.redirect_uri = Some(redirect_uri.into());
        self
    }

    /// Send the request to Slack and try to convert the response to a AccessResponse
    pub fn send(&mut self) -> Result<OauthAccessResponse> {
        let payload = self.client.send(format!("{}?{:?}", "oauth.access", &self.params))?;
        payload.to_type::<OauthAccessResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn oauth_access<S: Into<String>>(&mut self, client_id: S, client_secret: S, code: S) -> AccessBuilder {
        AccessBuilder::default(self,
        client_id.into(),
        client_secret.into(),
        code.into(),
        )
    }
}