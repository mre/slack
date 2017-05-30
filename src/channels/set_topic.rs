//! # channels_set_topic
//!
//! This method is used to change the topic of a channel. The calling user must be a member of the channel.  
//! See https://api.slack.com/methods/channels.setTopic  
//!
//! ## Required arguments: 
//!  `channel` (String)  
//!  Channel to set the topic of.  
//!  Example: C1234567890  
//!  `topic` (String)  
//!  The new topic.  
//!  Example: My Topic


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
pub struct SetTopicOptions {
/// Channel to set the topic of.
    pub channel: String,
/// The new topic.
    pub topic: String,

}

impl<'a> SetTopicOptions {

    /// Create a new instance of SetTopicOptions 
    fn new<S: Into<String>>(channel: S, topic: S) -> SetTopicOptions {
        SetTopicOptions { 
        channel: channel.into(),
        topic: topic.into(),
             ..Default::default()
        }
    }
}

impl From<SetTopicOptions> for String {
    fn from(options: SetTopicOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The SetTopicBuilder provides a fluid interface to create
/// objects of type SetTopicOptions
pub struct SetTopicBuilder<'a> {
    client: &'a mut Client,
    params: SetTopicOptions,
}

impl<'a> SetTopicBuilder<'a> {

    /// Create a default SetTopicBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, channel: S, topic: S) -> SetTopicBuilder<'a> {
        SetTopicBuilder {
            client: client,
            params: SetTopicOptions::new(
            channel,
            topic,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a SetTopicResponse
    pub fn send(&mut self) -> Result<ChannelsSetTopicResponse> {
        let payload = self.client.send(format!("{}?{:?}", "channels.setTopic", &self.params))?;
        payload.to_type::<ChannelsSetTopicResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn channels_set_topic<S: Into<String>>(&mut self, channel: S, topic: S) -> SetTopicBuilder {
        SetTopicBuilder::default(self,
        channel.into(),
        topic.into(),
        )
    }
}