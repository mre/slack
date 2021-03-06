//! Slack Rust Client
//!
//! A Rust Client for the Slack Web API.
//! It aims to be ergonomic and full-featured. It supports all Slack Web API methods.
//! The client is almost fully autogenerated and automatically built.
//!
//! # Configuration
//!
//! Set it up in your `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! slack-rs = "*"
//! ```
//!
//!  Congratulations! You're all set to talk to Slack.
//!
//! # Examples
//!
//! You can use it like so:
//!
//! ```
//! use slack_rs::Client;
//! let mut slack = Client::new("token");
//! let response = slack.im_history("channel_name").latest(123.1).send();
//! ```
//!
//! **Alternatively** you can construct and send Options directly:
//!
//! ```
//! use slack_rs::{Client, im};
//!
//! let mut slack = Client::new("token");
//! let response = slack.send(im::HistoryOptions {
//!   channel: "hello".to_string(),
//!   inclusive: Some(true),
//!   ..Default::default()
//! });
//! ```
//!
//! In both cases the result will be the same.
//! For more details check out the documentation for each method below.

#![cfg_attr(feature = "strict", deny(missing_docs))]
#![cfg_attr(feature = "strict", deny(warnings))]

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate hyper;

#[cfg(test)]
#[macro_use]
extern crate yup_hyper_mock as hyper_mock;

pub mod client;
pub mod types;
pub mod slack_types;
pub mod errors;

// Slack groups
pub mod api;
pub mod auth;
pub mod bots;
pub mod channels;
pub mod chat;
pub mod dnd;
pub mod emoji;
pub mod files;
pub mod groups;
pub mod im;
pub mod mpim;
pub mod oauth;
pub mod pins;
pub mod reactions;
pub mod reminders;
pub mod rtm;
pub mod search;
pub mod stars;
pub mod team;
pub mod usergroups;
pub mod users;

pub use client::Client;
