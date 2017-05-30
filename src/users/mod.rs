//! This module provides support for the slack `users` group of methods.
//! Every group is located in a separate Rust module.
//! Each module has its own mod.rs which reexports the methods
//! and their Option types. This makes using the client easier.

/// This method lets you find out information about a user's presence.  
/// Consult the presence documentation for more details. 
pub mod get_presence;
pub use self::get_presence::GetPresenceOptions;

/// After your Slack app is awarded an identity token through Sign in with Slack, use this method to retrieve a user's identity. 
pub mod identity;
pub use self::identity::IdentityOptions;

/// This method returns information about a team member. 
pub mod info;
pub use self::info::InfoOptions;

/// This method returns a list of all users in the team. This includes deleted/deactivated users. 
pub mod list;
pub use self::list::ListOptions;

/// This method is used to get the profile information for a user. 
pub mod profile_get;
pub use self::profile_get::ProfileGetOptions;

/// This method is used to set the profile information for a user. 
pub mod profile_set;
pub use self::profile_set::ProfileSetOptions;

/// This method lets the slack messaging server know that the authenticated user  
/// is currently active. Consult the presence documentation for  
/// more details. 
pub mod set_active;
pub use self::set_active::SetActiveOptions;

/// This method lets you set the calling user's manual presence.  
/// Consult the presence documentation for more details. 
pub mod set_presence;
pub use self::set_presence::SetPresenceOptions;