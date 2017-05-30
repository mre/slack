//! This module provides support for the slack `mpim` group of methods.
//! Every group is located in a separate Rust module.
//! Each module has its own mod.rs which reexports the methods
//! and their Option types. This makes using the client easier.

/// This method closes a multiparty direct message channel. 
pub mod close;
pub use self::close::CloseOptions;

/// This method returns a portion of messages/events from the specified multiparty direct message channel.  
/// To read the entire history for a multiparty direct message, call the method with no latest or  
/// oldest arguments, and then continue paging using the instructions below. 
pub mod history;
pub use self::history::HistoryOptions;

/// This method returns a list of all multiparty direct message channels that the user has. 
pub mod list;
pub use self::list::ListOptions;

/// This method moves the read cursor in a multiparty direct message channel. 
pub mod mark;
pub use self::mark::MarkOptions;

/// This method opens a multiparty direct message. 
pub mod open;
pub use self::open::OpenOptions;