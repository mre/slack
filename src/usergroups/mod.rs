//! This module provides support for the slack `usergroups` group of methods.
//! Every group is located in a separate Rust module.
//! Each module has its own mod.rs which reexports the methods
//! and their Option types. This makes using the client easier.

/// This method is used to create a User Group. 
pub mod create;
pub use self::create::CreateOptions;

/// This method disables an existing User Group. 
pub mod disable;
pub use self::disable::DisableOptions;

/// This method enables a User Group which was previously disabled. 
pub mod enable;
pub use self::enable::EnableOptions;

/// This method returns a list of all User Groups in the team. This can optionally include disabled User Groups. 
pub mod list;
pub use self::list::ListOptions;

/// This method updates the properties of an existing User Group. 
pub mod update;
pub use self::update::UpdateOptions;

/// This method returns a list of all users within a User Group. 
pub mod users_list;
pub use self::users_list::UsersListOptions;

/// This method updates the list of users that belong to a User Group. This method replaces all users in a User Group with the list of users provided in the users parameter. 
pub mod users_update;
pub use self::users_update::UsersUpdateOptions;