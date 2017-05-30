//! This module provides support for the slack `pins` group of methods.
//! Every group is located in a separate Rust module.
//! Each module has its own mod.rs which reexports the methods
//! and their Option types. This makes using the client easier.

/// This method pins an item (file, file comment, channel message, or group message) to a particular channel.  
/// The channel argument is required and one of file, file_comment, or timestamp must also be specified. 
pub mod add;
pub use self::add::AddOptions;

/// This method lists the items pinned to a channel. 
pub mod list;
pub use self::list::ListOptions;

/// This method un-pins an item (file, file comment, channel message, or group message) from a channel.  
/// The channel argument is required and one of file, file_comment, or timestamp must also be specified. 
pub mod remove;
pub use self::remove::RemoveOptions;