//! This module provides support for the slack `search` group of methods.
//! Every group is located in a separate Rust module.
//! Each module has its own mod.rs which reexports the methods
//! and their Option types. This makes using the client easier.

/// This method allows users and applications to search both messages and files in a single call. 
pub mod all;
pub use self::all::AllOptions;

/// This method returns files matching a search query. 
pub mod files;
pub use self::files::FilesOptions;

/// This method returns messages matching a search query. 
pub mod messages;
pub use self::messages::MessagesOptions;