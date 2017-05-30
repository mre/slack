//! This module provides support for the slack `oauth` group of methods.
//! Every group is located in a separate Rust module.
//! Each module has its own mod.rs which reexports the methods
//! and their Option types. This makes using the client easier.

/// This method allows you to exchange a temporary OAuth code for an API access token.  
/// This is used as part of the OAuth authentication flow. 
pub mod access;
pub use self::access::AccessOptions;