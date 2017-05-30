//! This module provides support for the slack `api` group of methods.
//! Every group is located in a separate Rust module.
//! Each module has its own mod.rs which reexports the methods
//! and their Option types. This makes using the client easier.

/// This method helps you test your calling code. 
pub mod test;
pub use self::test::TestOptions;