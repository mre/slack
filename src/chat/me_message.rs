//! # chat_me_message
//!
//! This method sends a me message to a channel from the calling user.  
//! See https://api.slack.com/methods/chat.meMessage  
//!
//! ## Required arguments: 
//!  `channel` (String)  
//!  Channel to send message to. Can be a public channel, private group or IM channel. Can be an encoded ID, or a name.  
//!  Example: C1234567890  
//!  `text` (String)  
//!  Text of the message to send.  
//!  Example: Hello world


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
pub struct MeMessageOptions {
/// Channel to send message to. Can be a public channel, private group or IM channel. Can be an encoded ID, or a name.
    pub channel: String,
/// Text of the message to send.
    pub text: String,

}

impl<'a> MeMessageOptions {

    /// Create a new instance of MeMessageOptions 
    fn new<S: Into<String>>(channel: S, text: S) -> MeMessageOptions {
        MeMessageOptions { 
        channel: channel.into(),
        text: text.into(),
             ..Default::default()
        }
    }
}

impl From<MeMessageOptions> for String {
    fn from(options: MeMessageOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The MeMessageBuilder provides a fluid interface to create
/// objects of type MeMessageOptions
pub struct MeMessageBuilder<'a> {
    client: &'a mut Client,
    params: MeMessageOptions,
}

impl<'a> MeMessageBuilder<'a> {

    /// Create a default MeMessageBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, channel: S, text: S) -> MeMessageBuilder<'a> {
        MeMessageBuilder {
            client: client,
            params: MeMessageOptions::new(
            channel,
            text,
            ),
        }
    }


    /// Send the request to Slack and try to convert the response to a MeMessageResponse
    pub fn send(&mut self) -> Result<ChatMeMessageResponse> {
        let payload = self.client.send(format!("{}?{:?}", "chat.meMessage", &self.params))?;
        payload.to_type::<ChatMeMessageResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn chat_me_message<S: Into<String>>(&mut self, channel: S, text: S) -> MeMessageBuilder {
        MeMessageBuilder::default(self,
        channel.into(),
        text.into(),
        )
    }
}