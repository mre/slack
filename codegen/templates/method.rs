//! # {{ method.group }}_{{ method.name }}
//!
{% for line in method.desc %}
//! {{ line }}  
{% endfor %}  
//! See https://api.slack.com/methods/{{ method.endpoint }}  
//!
{% if method.required_args %}
//! ## Required arguments: 
{% for required_arg in method.required_args %}
//!  `{{ required_arg.name }}` ({{ required_arg._type }})  
//!  {{ required_arg.desc }}  
//!  Example: {{ required_arg.example }}  
{% endfor %}
{% endif %}
{% if method.optional_args %}
//!
//! ## Optional arguments: 
{% for optional_arg in method.optional_args %}
//!
//!  `{{ optional_arg.name }}` ({{ optional_arg._type }})  
//!  {{ optional_arg.desc }}  
//!  Example: {{ optional_arg.example }}  
{% endfor %}
{% endif %}

// Warning! This file was auto-generated from the Slack API specification.
// Don't modify it directly. Modify the template that generates this file instead.

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate serde_urlencoded;

use client::Client;

use types::*;
use slack_types::*;

use errors::*;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct {{ method.title }}Options {
{% for arg in method.required_args %}
    /// {{ arg.desc }}
    pub {{ arg.name }}: {{ arg._type }},
{% endfor %}
{% for arg in method.optional_args %}
    /// {{ arg.desc }}
    pub {{ arg.name }}: Option<{{ arg._type }}>,
{% endfor %}
}

impl<'a> {{ method.title }}Options {

    /// Create a new instance of {{ method.title }}Options 
    fn new{{ method.trait_bounds }}({{ method.required_args_types_list | join(sep=", ") }}) -> {{ method.title }}Options {
        {{ method.title }}Options { 
        {% for arg in method.required_args %}
            {% if arg._type == "String" %}
              {{ arg.name }}: {{ arg.name }}.into(),
            {% else %}
              {{ arg.name }}: {{ arg.name }},
            {% endif %}
        {% endfor %}
             ..Default::default()
        }
    }
}

impl From<{{ method.title }}Options> for String {
    fn from(options: {{ method.title }}Options) -> Self {
        // This should never fail, so calling unwrap() on the result.
        // There might be a better way, though.
        serde_urlencoded::to_string(&options).unwrap()
    }
}

/// The {{ method.title }}Builder provides a fluid interface to create
/// objects of type {{ method.title }}Options
pub struct {{ method.title }}Builder<'a> {
    client: &'a mut Client,
    params: {{ method.title }}Options,
}

impl<'a> {{ method.title }}Builder<'a> {

    /// Create a default {{ method.title }}Builder object
    pub fn default{{ method.trait_bounds }}(client: &'a mut Client, {{ method.required_args_types_list | join(sep=", ") }}) -> {{ method.title }}Builder<'a> {
        {{ method.title }}Builder {
            client: client,
            params: {{ method.title }}Options::new(
            {% for arg in method.required_args %}
              {{ arg.name }},
            {% endfor %}
            ),
        }
    }
{% for arg in method.optional_args %}

    /// {{ arg.desc }}
    {% if arg._type == "String" %}
    pub fn {{ arg.name }}<S: Into<String>>(&'a mut self, {{ arg.name }}: S) -> &mut {{ method.title}}Builder {
        self.params.{{ arg.name }} = Some({{ arg.name }}.into());
    {% else %}
    pub fn {{ arg.name }}(&'a mut self, {{ arg.name }}: {{ arg._type }}) -> &mut {{ method.title}}Builder {
        self.params.{{ arg.name }} = Some({{ arg.name }});
    {% endif %}
        self
    }
{% endfor %}

    /// Send the request to Slack and try to convert the response to a {{ method.title }}Response
    pub fn send(&mut self) -> Result<{{ method.group | title }}{{ method.title }}Response> {
        let payload = self.client.send(format!("{}?{:?}", "{{ method.endpoint }}", &self.params))?;
        payload.to_type::<{{ method.group | title }}{{ method.title }}Response>().or_else(|_| Err(self.client.read_error(&payload)))
    }
}

impl Client {
    /// A shorthand method to call the Slack API directly
    pub fn {{ method.full_name }}{{ method.trait_bounds }}(&mut self, {{ method.required_args_types_list | join(sep=", ") }}) -> {{ method.title }}Builder {
        {{ method.title }}Builder::default(self,
        {% for arg in method.required_args %}
            {% if arg._type == "String" %}
              {{ arg.name }}.into(),
            {% else %}
              {{ arg.name }},
            {% endif %}
        {% endfor %}
        )
    }
}