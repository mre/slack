//! This module provides support for the slack `stars` group of methods.
//! Every group is located in a separate Rust module.
//! Each module has its own mod.rs which reexports the methods
//! and their Option types. This makes using the client easier.

/// This method adds a star to an item (message, file, file comment, channel, private group, or DM) on behalf of the authenticated user.  
/// One of file, file_comment, channel, or the combination of channel and timestamp must be specified. 
pub mod add;
pub use self::add::AddOptions;

/// This method lists the items starred by the authed user. 
pub mod list;
pub use self::list::ListOptions;

/// This method removes a star from an item (message, file, file comment, channel, private group, or DM) on behalf of the authenticated user.  
/// One of file, file_comment, channel, or the combination of channel and timestamp must be specified. 
pub mod remove;
pub use self::remove::RemoveOptions;