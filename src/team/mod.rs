//! This module provides support for the slack `team` group of methods.
//! Every group is located in a separate Rust module.
//! Each module has its own mod.rs which reexports the methods
//! and their Option types. This makes using the client easier.

/// This method is used to get the access logs for users on a team. 
pub mod access_logs;
pub use self::access_logs::AccessLogsOptions;

/// This method lists billable information for each user on the team. Currently this consists solely of whether the user is  
/// subject to billing per Slack's Fair Billing policy. 
pub mod billable_info;
pub use self::billable_info::BillableInfoOptions;

/// This method provides information about your team. 
pub mod info;
pub use self::info::InfoOptions;

/// This method lists the integration activity logs for a team, including when integrations are added, modified and removed. This method can only be called by Admins. 
pub mod integration_logs;
pub use self::integration_logs::IntegrationLogsOptions;

/// This method is used to get the profile field definitions for this team. 
pub mod profile_get;
pub use self::profile_get::ProfileGetOptions;