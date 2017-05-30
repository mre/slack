//! # auth_test
//!
//! This method checks authentication and tells you who you are.  
//! See https://api.slack.com/methods/auth.test  
//!



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
pub struct TestOptions {


}

impl<'a> TestOptions {

    /// Create a new instance of TestOptions 
    fn new<>() -> TestOptions {
        TestOptions { 
        
             ..Default::default()
        }
    }
}

impl From<TestOptions> for String {
    fn from(options: TestOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The TestBuilder provides a fluid interface to create
/// objects of type TestOptions
pub struct TestBuilder<'a> {
    client: &'a mut Client,
    params: TestOptions,
}

impl<'a> TestBuilder<'a> {

    /// Create a default TestBuilder object
    pub fn default<>(client: &'a mut Client, ) -> TestBuilder<'a> {
        TestBuilder {
            client: client,
            params: TestOptions::new(
            
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a TestResponse
    pub fn send(&mut self) -> Result<AuthTestResponse> {
        let payload = self.client.send(format!("{}?{:?}", "auth.test", &self.params))?;
        payload.to_type::<AuthTestResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn auth_test<>(&mut self, ) -> TestBuilder {
        TestBuilder::default(self,
        
        )
    }
}