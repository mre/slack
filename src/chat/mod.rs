//! This module provides support for the slack `chat` group of methods.
//! Every group is located in a separate Rust module.
//! Each module has its own mod.rs which reexports the methods
//! and their Option types. This makes using the client easier.

/// This method deletes a message from a channel. 
pub mod delete;
pub use self::delete::DeleteOptions;

/// This method sends a me message to a channel from the calling user. 
pub mod me_message;
pub use self::me_message::MeMessageOptions;

/// This method posts a message to a public channel, private channel, or direct message/IM channel. 
pub mod post_message;
pub use self::post_message::PostMessageOptions;

/// This method updates a message in a channel. Though related to chat.postMessage, some parameters of chat.update are handled differently. 
pub mod update;
pub use self::update::UpdateOptions;