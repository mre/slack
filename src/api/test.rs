//! # api_test
//!
//! This method helps you test your calling code.  
//! See https://api.slack.com/methods/api.test  
//!

//!
//! ## Optional arguments: 
//!
//!  `error` (String)  
//!  Error response to return.  
//!  Example: my_error  
//!
//!  `foo` (String)  
//!  example property to return.  
//!  Example: bar

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

/// Error response to return.
    pub error: Option<String>,
/// example property to return.
    pub foo: Option<String>,
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
/// Error response to return.
    pub fn error<S: Into<String>>(&'a mut self, error: S) -> &mut TestBuilder {
        self.params.error = Some(error.into());
        self
    }
/// example property to return.
    pub fn foo<S: Into<String>>(&'a mut self, foo: S) -> &mut TestBuilder {
        self.params.foo = Some(foo.into());
        self
    }

    /// Send the request to Slack and try to convert the response to a TestResponse
    pub fn send(&mut self) -> Result<ApiTestResponse> {
        let payload = self.client.send(format!("{}?{:?}", "api.test", &self.params))?;
        payload.to_type::<ApiTestResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn api_test<>(&mut self, ) -> TestBuilder {
        TestBuilder::default(self,
        
        )
    }
}