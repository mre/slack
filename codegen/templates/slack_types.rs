//! Slack Response types. These are auto-generated from the response examples of the official API documentation.

// Example entry:
// #[derive(Debug, PartialEq, Serialize, Deserialize)]
// pub struct ImListResponse {}

{% for _, group in groups %}
{% for method in group.methods %}

/// Response type for {{ method.full_name }}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct {{ method.group | title }}{{ method.title }}Response {}

{% endfor %}
{% endfor %}