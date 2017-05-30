//! This module provides support for the slack `reminders` group of methods.
//! Every group is located in a separate Rust module.
//! Each module has its own mod.rs which reexports the methods
//! and their Option types. This makes using the client easier.

/// This method creates a reminder. 
pub mod add;
pub use self::add::AddOptions;

/// This method completes a reminder. 
pub mod complete;
pub use self::complete::CompleteOptions;

/// This method deletes a reminder. 
pub mod delete;
pub use self::delete::DeleteOptions;

/// This method returns information about a reminder. 
pub mod info;
pub use self::info::InfoOptions;

/// This method lists all reminders created by or for a given user. 
pub mod list;
pub use self::list::ListOptions;