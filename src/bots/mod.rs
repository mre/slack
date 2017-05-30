//! This module provides support for the slack `bots` group of methods.
//! Every group is located in a separate Rust module.
//! Each module has its own mod.rs which reexports the methods
//! and their Option types. This makes using the client easier.

/// This method returns information about a bot user. 
pub mod info;
pub use self::info::InfoOptions;