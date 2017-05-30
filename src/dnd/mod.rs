//! This module provides support for the slack `dnd` group of methods.
//! Every group is located in a separate Rust module.
//! Each module has its own mod.rs which reexports the methods
//! and their Option types. This makes using the client easier.

/// Ends the user's currently scheduled Do Not Disturb session immediately. 
pub mod end_dnd;
pub use self::end_dnd::EndDndOptions;

/// Ends the current user's snooze mode immediately. 
pub mod end_snooze;
pub use self::end_snooze::EndSnoozeOptions;

/// Provides information about a user's current Do Not Disturb settings. 
pub mod info;
pub use self::info::InfoOptions;

/// Adjusts the snooze duration for a user's Do Not Disturb settings. If a snooze session is not already active for the user, invoking this method will begin one for the specified duration. 
pub mod set_snooze;
pub use self::set_snooze::SetSnoozeOptions;

/// Provides information about the current Do Not Disturb settings for users of a Slack team. 
pub mod team_info;
pub use self::team_info::TeamInfoOptions;