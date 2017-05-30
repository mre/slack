//! This module provides support for the slack `files` group of methods.
//! Every group is located in a separate Rust module.
//! Each module has its own mod.rs which reexports the methods
//! and their Option types. This makes using the client easier.

/// Add a comment to an existing file. 
pub mod comments_add;
pub use self::comments_add::CommentsAddOptions;

/// Delete an existing comment on a file. Only the original author of the comment or a Team Administrator may delete a file comment. 
pub mod comments_delete;
pub use self::comments_delete::CommentsDeleteOptions;

/// Edit an existing comment on a file. Only the user who created a comment may make edits. Teams may configure a limited time window during which file comment edits are allowed. 
pub mod comments_edit;
pub use self::comments_edit::CommentsEditOptions;

/// This method deletes a file from your team. 
pub mod delete;
pub use self::delete::DeleteOptions;

/// This method returns information about a file in your team. 
pub mod info;
pub use self::info::InfoOptions;

/// This method returns a list of files within the team. It can be filtered and sliced in various ways. 
pub mod list;
pub use self::list::ListOptions;

/// This method disables public/external sharing for a file. 
pub mod revoke_public_url;
pub use self::revoke_public_url::RevokePublicURLOptions;

/// This method enables public/external sharing for a file. 
pub mod shared_public_url;
pub use self::shared_public_url::SharedPublicURLOptions;

/// This method allows you to create or upload an existing file. 
pub mod upload;
pub use self::upload::UploadOptions;