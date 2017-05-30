//! This module provides support for the slack `rtm` group of methods.
//! Every group is located in a separate Rust module.
//! Each module has its own mod.rs which reexports the methods
//! and their Option types. This makes using the client easier.

/// This method starts a Real Time Messaging API session. Refer to the  
/// RTM API documentation for full details on how to use the RTM API. 
pub mod start;
pub use self::start::StartOptions;