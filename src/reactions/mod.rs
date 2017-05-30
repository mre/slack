//! This module provides support for the slack `reactions` group of methods.
//! Every group is located in a separate Rust module.
//! Each module has its own mod.rs which reexports the methods
//! and their Option types. This makes using the client easier.

/// This method adds a reaction (emoji) to an item (file, file comment, channel message, group message, or direct message).  
/// One of file, file_comment, or the combination of channel and timestamp must be specified. 
pub mod add;
pub use self::add::AddOptions;

/// This method returns a list of all reactions for a single item (file, file comment, channel message, group message, or direct message). 
pub mod get;
pub use self::get::GetOptions;

/// This method returns a list of all items (file, file comment, channel message, group message, or direct message) reacted to by a user. 
pub mod list;
pub use self::list::ListOptions;

/// This method removes a reaction (emoji) from an item (file, file comment, channel message, group message, or direct message).  
/// One of file, file_comment, or the combination of channel and timestamp must be specified. 
pub mod remove;
pub use self::remove::RemoveOptions;