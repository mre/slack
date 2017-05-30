//! This module provides support for the slack `groups` group of methods.
//! Every group is located in a separate Rust module.
//! Each module has its own mod.rs which reexports the methods
//! and their Option types. This makes using the client easier.

/// This method archives a private channel. 
pub mod archive;
pub use self::archive::ArchiveOptions;

/// This method closes a private channel. 
pub mod close;
pub use self::close::CloseOptions;

/// This method creates a private channel. 
pub mod create;
pub use self::create::CreateOptions;

/// This method takes an existing private channel and performs the following steps: 
pub mod create_child;
pub use self::create_child::CreateChildOptions;

/// This method returns a portion of messages/events from the specified private channel.  
/// To read the entire history for a private channel, call the method with no latest or  
/// oldest arguments, and then continue paging using the instructions below. 
pub mod history;
pub use self::history::HistoryOptions;

/// This method returns information about a private channel. 
pub mod info;
pub use self::info::InfoOptions;

/// This method is used to invite a user to a private channel. The calling user must be a member of the private channel. 
pub mod invite;
pub use self::invite::InviteOptions;

/// This method allows a user to remove another member from a private channel. 
pub mod kick;
pub use self::kick::KickOptions;

/// This method is used to leave a private channel. 
pub mod leave;
pub use self::leave::LeaveOptions;

/// This method returns a list of private channels in the team that the caller is in and archived groups that the caller was in.  
/// The list of (non-deactivated) members in each private channel is also returned. 
pub mod list;
pub use self::list::ListOptions;

/// This method moves the read cursor in a private channel. 
pub mod mark;
pub use self::mark::MarkOptions;

/// This method opens a private channel. 
pub mod open;
pub use self::open::OpenOptions;

/// This method renames a private channel. 
pub mod rename;
pub use self::rename::RenameOptions;

/// This method is used to change the purpose of a private channel. The calling user must be a member of the private channel. 
pub mod set_purpose;
pub use self::set_purpose::SetPurposeOptions;

/// This method is used to change the topic of a private channel. The calling user must be a member of the private channel. 
pub mod set_topic;
pub use self::set_topic::SetTopicOptions;

/// This method unarchives a private channel. 
pub mod unarchive;
pub use self::unarchive::UnarchiveOptions;