//! The Client is the main interaction point with the library.
//! It sends requests and deserializes Slack JSON responses
//! into semantic Rust types. It's also responsible for error handling.
//!
//! To create a `Client`, the token needs to be specified.
//!
//! Each Slack API operation is defined as a method on `Client`.  Any
//! compulsory parameters must be given as arguments to this method.
//! It returns an operation builder that can be used to add any
//! optional parameters.
//!
//! Finally `send` is called to submit the operation:

extern crate serde_json;

use errors::*;
pub use im::history;
use std;
use errors::SlackErrorMessage;
pub use hyper::Client as HyperClient;
use hyper::net::{NetworkConnector, NetworkStream};

use types::*;

static API_BASE_URL: &'static str = "https://slack.com/api/";

/// Client holds the connection to Slack and the Slack token
pub struct Client {
    hyper_client: HyperClient,
    token: String,
}

impl Client {
    /// Creates a new Slack client
    /// You need to pass your Slack API token.
    pub fn new<T>(token: T) -> Client
        where T: Into<String>
    {
        Client {
            token: token.into(),
            hyper_client: HyperClient::new(),
        }
    }

    /// Set a different connectior for the client.
    /// This is mainly useful for testing (mocking HTTP connections).
    pub fn with_connector<T, C, S>(token: T, connector: C) -> Client
        where T: Into<String>,
              C: NetworkConnector<Stream = S> + Send + Sync + 'static,
              S: NetworkStream + Send
    {
        let hyper_client = HyperClient::with_connector(connector);
        Client {
            token: token.into(),
            hyper_client: hyper_client,
        }
    }

    /// Convert a Slack error given as a string to an error defined by this library
    /// which can be matched on.
    pub fn read_error(&self, response: &Response) -> Error {
        match serde_json::from_str::<SlackErrorMessage>(&response.body) {
            Ok(slack_error) => Error::from_kind(ErrorKind::Slack(slack_error.error)),
            Err(e) => Error::from_kind(ErrorKind::Serialization(e.to_string())),
        }
    }

    /// Send a request to the Slack API and parse the response
    pub fn send<T>(&self, endpoint: T) -> Result<Response>
        where T: Into<String>
    {
        let url = self.url(endpoint.into());
        let response = self.hyper_client.get(&url).send()?;
        Response::from_hyper_response(response)
    }

    fn url<T>(&self, endpoint: T) -> String
        where T: Into<String> + std::fmt::Display
    {
        format!("{}{}&token={}", API_BASE_URL, endpoint, self.token)
    }
}