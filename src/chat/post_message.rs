//! # chat_post_message
//!
//! This method posts a message to a public channel, private channel, or direct message/IM channel.  
//! See https://api.slack.com/methods/chat.postMessage  
//!
//! ## Required arguments: 
//!  `channel` (String)  
//!  Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name. See below for more details.  
//!  Example: C1234567890  
//!  `text` (String)  
//!  Text of the message to send. See below for an explanation of formatting. This field is usually required, unless you're providing only attachments instead.  
//!  Example: Hello world
//!
//! ## Optional arguments: 
//!
//!  `as_user` (bool)  
//!  Pass true to post the message as the authed user, instead of as a bot. Defaults to false. See authorship below.  
//!  Example: true  
//!
//!  `attachments` (String)  
//!  Structured message attachments.  
//!  Example: [{"pretext": "pre-hello", "text": "text-world"}]  
//!
//!  `icon_emoji` (String)  
//!  emoji to use as the icon for this message. Overrides icon_url. Must be used in conjunction with as_user set to false, otherwise ignored. See authorship below.  
//!  Example: :chart_with_upwards_trend:  
//!
//!  `icon_url` (String)  
//!  URL to an image to use as the icon for this message. Must be used in conjunction with as_user set to false, otherwise ignored. See authorship below.  
//!  Example: http://lorempixel.com/48/48  
//!
//!  `link_names` (bool)  
//!  Find and link channel names and usernames.  
//!  Example: 1  
//!
//!  `parse` (String)  
//!  Change how messages are treated. Defaults to none. See below.  
//!  Example: full  
//!
//!  `unfurl_links` (bool)  
//!  Pass true to enable unfurling of primarily text-based content.  
//!  Example: true  
//!
//!  `unfurl_media` (bool)  
//!  Pass false to disable unfurling of media content.  
//!  Example: false  
//!
//!  `username` (String)  
//!  Set your bot's user name. Must be used in conjunction with as_user set to false, otherwise ignored. See authorship below.  
//!  Example: My Bot

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
pub struct PostMessageOptions {
/// Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name. See below for more details.
    pub channel: String,
/// Text of the message to send. See below for an explanation of formatting. This field is usually required, unless you're providing only attachments instead.
    pub text: String,
/// Pass true to post the message as the authed user, instead of as a bot. Defaults to false. See authorship below.
    pub as_user: Option<bool>,
/// Structured message attachments.
    pub attachments: Option<String>,
/// emoji to use as the icon for this message. Overrides icon_url. Must be used in conjunction with as_user set to false, otherwise ignored. See authorship below.
    pub icon_emoji: Option<String>,
/// URL to an image to use as the icon for this message. Must be used in conjunction with as_user set to false, otherwise ignored. See authorship below.
    pub icon_url: Option<String>,
/// Find and link channel names and usernames.
    pub link_names: Option<bool>,
/// Change how messages are treated. Defaults to none. See below.
    pub parse: Option<String>,
/// Pass true to enable unfurling of primarily text-based content.
    pub unfurl_links: Option<bool>,
/// Pass false to disable unfurling of media content.
    pub unfurl_media: Option<bool>,
/// Set your bot's user name. Must be used in conjunction with as_user set to false, otherwise ignored. See authorship below.
    pub username: Option<String>,
}

impl<'a> PostMessageOptions {

    /// Create a new instance of PostMessageOptions 
    fn new<S: Into<String>>(channel: S, text: S) -> PostMessageOptions {
        PostMessageOptions { 
        channel: channel.into(),
        text: text.into(),
             ..Default::default()
        }
    }
}

impl From<PostMessageOptions> for String {
    fn from(options: PostMessageOptions) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The PostMessageBuilder provides a fluid interface to create
/// objects of type PostMessageOptions
pub struct PostMessageBuilder<'a> {
    client: &'a mut Client,
    params: PostMessageOptions,
}

impl<'a> PostMessageBuilder<'a> {

    /// Create a default PostMessageBuilder object
    pub fn default<S: Into<String>>(client: &'a mut Client, channel: S, text: S) -> PostMessageBuilder<'a> {
        PostMessageBuilder {
            client: client,
            params: PostMessageOptions::new(
            channel,
            text,
            ),
        }
    }
/// Pass true to post the message as the authed user, instead of as a bot. Defaults to false. See authorship below.
    pub fn as_user(&'a mut self, as_user: bool) -> &mut PostMessageBuilder {
        self.params.as_user = Some(as_user);
        self
    }
/// Structured message attachments.
    pub fn attachments<S: Into<String>>(&'a mut self, attachments: S) -> &mut PostMessageBuilder {
        self.params.attachments = Some(attachments.into());
        self
    }
/// emoji to use as the icon for this message. Overrides icon_url. Must be used in conjunction with as_user set to false, otherwise ignored. See authorship below.
    pub fn icon_emoji<S: Into<String>>(&'a mut self, icon_emoji: S) -> &mut PostMessageBuilder {
        self.params.icon_emoji = Some(icon_emoji.into());
        self
    }
/// URL to an image to use as the icon for this message. Must be used in conjunction with as_user set to false, otherwise ignored. See authorship below.
    pub fn icon_url<S: Into<String>>(&'a mut self, icon_url: S) -> &mut PostMessageBuilder {
        self.params.icon_url = Some(icon_url.into());
        self
    }
/// Find and link channel names and usernames.
    pub fn link_names(&'a mut self, link_names: bool) -> &mut PostMessageBuilder {
        self.params.link_names = Some(link_names);
        self
    }
/// Change how messages are treated. Defaults to none. See below.
    pub fn parse<S: Into<String>>(&'a mut self, parse: S) -> &mut PostMessageBuilder {
        self.params.parse = Some(parse.into());
        self
    }
/// Pass true to enable unfurling of primarily text-based content.
    pub fn unfurl_links(&'a mut self, unfurl_links: bool) -> &mut PostMessageBuilder {
        self.params.unfurl_links = Some(unfurl_links);
        self
    }
/// Pass false to disable unfurling of media content.
    pub fn unfurl_media(&'a mut self, unfurl_media: bool) -> &mut PostMessageBuilder {
        self.params.unfurl_media = Some(unfurl_media);
        self
    }
/// Set your bot's user name. Must be used in conjunction with as_user set to false, otherwise ignored. See authorship below.
    pub fn username<S: Into<String>>(&'a mut self, username: S) -> &mut PostMessageBuilder {
        self.params.username = Some(username.into());
        self
    }

    /// Send the request to Slack and try to convert the response to a PostMessageResponse
    pub fn send(&mut self) -> Result<ChatPostMessageResponse> {
        let payload = self.client.send(format!("{}?{:?}", "chat.postMessage", &self.params))?;
        payload.to_type::<ChatPostMessageResponse>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn chat_post_message<S: Into<String>>(&mut self, channel: S, text: S) -> PostMessageBuilder {
        PostMessageBuilder::default(self,
        channel.into(),
        text.into(),
        )
    }
}