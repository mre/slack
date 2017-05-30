//! This module provides support for the slack `emoji` group of methods.
//! Every group is located in a separate Rust module.
//! Each module has its own mod.rs which reexports the methods
//! and their Option types. This makes using the client easier.

/// This method lists the custom emoji for a team. 
pub mod list;
pub use self::list::ListOptions;