//! This module provides support for the slack `auth` group of methods.
//! Every group is located in a separate Rust module.
//! Each module has its own mod.rs which reexports the methods
//! and their Option types. This makes using the client easier.

/// This method revokes an access token. Use it when you no longer need a token. For example, with a Sign In With Slack app, call this to log a user out. 
pub mod revoke;
pub use self::revoke::RevokeOptions;

/// This method checks authentication and tells you who you are. 
pub mod test;
pub use self::test::TestOptions;