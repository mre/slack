//! This module provides support for the slack `{{ group.name }}` group of methods.
//! Every group is located in a separate Rust module.
//! Each module has its own mod.rs which reexports the methods
//! and their Option types. This makes using the client easier.

{% for method in group.methods %}

{% for line in method.desc %}
/// {{ line }}  
{% endfor %} 
pub mod {{ method.name }};
pub use self::{{ method.name }}::{{ method.title }}Options;

{% endfor %}