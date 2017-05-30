//! # im_history
//!
//! This method returns a portion of messages/events from the specified direct message channel.  
//! To read the entire history for a direct message channel, call the method with no latest or  
//! oldest arguments, and then continue paging using the instructions below.  
//! See https://api.slack.com/methods/im.history  
//!
//! ## Required arguments: 
//!  `channel` (String)  
//!  Direct message channel to fetch history for.  
//!  Example: D1234567890
//!
//! ## Optional arguments: 
//!
//!  `inclusive` (bool)  
//!  Include messages with latest or oldest timestamp in results.  
//!  Example: 1  
//!
//!  `latest` (f64)  
//!  End of time range of messages to include in results.  
//!  Example: 1234567890.123456  
//!
//!  `oldest` (f64)  
//!  Start of time range of messages to include in results.  
//!  Example: 1234567890.123456  
//!
//!  `unreads` (bool)  
//!  Include unread_count_display in the output?.  
//!  Example: 1

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
pub struct HistoryOptions {
/// Direct message channel to fetch history for.
    pub channel: String,
/// Include messages with latest or oldest timestamp in results.
    pub inclusive: Option<bool>,
/// End of time range of messages to include in results.
    pub latest: Option<f64>,
/// Start of time range of messages to include in results.
    pub oldest: Option<f64>,
/// Include unread_count_display in the output?.
    pub unreads: Option<bool>,
}

impl<'a> HistoryOptions {

    /// Create a new instance of HistoryOptions 
    fn new<S: Into<String>>(channel: S) -> HistoryOptions {
        HistoryOptions { 
        channel: channel.into(),
             ..Default::default()
        }
    }
}

impl From<HistoryOptions> for String {
    fn from(options: HistoryOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The HistoryBuilder provides a fluid interface to create
/// objects of type HistoryOptions
pub struct HistoryBuilder<'a> {
    client: &'a mut Client,
    params: HistoryOptions,
}

impl<'a> HistoryBuilder<'a> {

    /// Create a default HistoryBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, channel: S) -> HistoryBuilder<'a> {
        HistoryBuilder {
            client: client,
            params: HistoryOptions::new(
            channel,
            ),
        }
    }
/// Include messages with latest or oldest timestamp in results.
    pub fn inclusive(&'a mut self, inclusive: bool) -> &mut HistoryBuilder {
        self.params.inclusive = Some(inclusive);
        self
    }
/// End of time range of messages to include in results.
    pub fn latest(&'a mut self, latest: f64) -> &mut HistoryBuilder {
        self.params.latest = Some(latest);
        self
    }
/// Start of time range of messages to include in results.
    pub fn oldest(&'a mut self, oldest: f64) -> &mut HistoryBuilder {
        self.params.oldest = Some(oldest);
        self
    }
/// Include unread_count_display in the output?.
    pub fn unreads(&'a mut self, unreads: bool) -> &mut HistoryBuilder {
        self.params.unreads = Some(unreads);
        self
    }

    /// Send the request to Slack and try to convert the response to a HistoryResponse
    pub fn send(&mut self) -> Result<ImHistoryResponse> {
        let payload = self.client.send(format!("{}?{:?}", "im.history", &self.params))?;
        payload.to_type::<ImHistoryResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn im_history<S: Into<String>>(&mut self, channel: S) -> HistoryBuilder {
        HistoryBuilder::default(self,
        channel.into(),
        )
    }
}