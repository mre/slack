//! This module provides support for the slack `channels` group of methods.
//! Every group is located in a separate Rust module.
//! Each module has its own mod.rs which reexports the methods
//! and their Option types. This makes using the client easier.

/// This method archives a channel. 
pub mod archive;
pub use self::archive::ArchiveOptions;

/// This method is used to create a channel. 
pub mod create;
pub use self::create::CreateOptions;

/// This method returns a portion of message events from the specified channel. 
pub mod history;
pub use self::history::HistoryOptions;

/// This method returns information about a team channel. 
pub mod info;
pub use self::info::InfoOptions;

/// This method is used to invite a user to a channel. The calling user must be a member of the channel. 
pub mod invite;
pub use self::invite::InviteOptions;

/// This method is used to join a channel. If the channel does not exist, it is  
/// created. 
pub mod join;
pub use self::join::JoinOptions;

/// This method allows a user to remove another member from a team channel. 
pub mod kick;
pub use self::kick::KickOptions;

/// This method is used to leave a channel. 
pub mod leave;
pub use self::leave::LeaveOptions;

/// This method returns a list of all channels in the team. This includes channels the caller is in, channels they are not currently in, and archived channels but does not include private channels. The number of (non-deactivated) members in each channel is also returned. 
pub mod list;
pub use self::list::ListOptions;

/// This method moves the read cursor in a channel. 
pub mod mark;
pub use self::mark::MarkOptions;

/// This method renames a team channel. 
pub mod rename;
pub use self::rename::RenameOptions;

/// This method is used to change the purpose of a channel. The calling user must be a member of the channel. 
pub mod set_purpose;
pub use self::set_purpose::SetPurposeOptions;

/// This method is used to change the topic of a channel. The calling user must be a member of the channel. 
pub mod set_topic;
pub use self::set_topic::SetTopicOptions;

/// This method unarchives a channel. The calling user is added to the channel. 
pub mod unarchive;
pub use self::unarchive::UnarchiveOptions;