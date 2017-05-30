use std::collections::HashMap;

/// The Argument type as defined in the JSON spec
#[derive(Debug, Serialize, Deserialize)]
pub struct SpecArgument {
    pub example: String,
    pub desc: String,
    pub required: bool,
}

/// The Method Error type as defined in the JSON spec
#[derive(Debug, Serialize, Deserialize)]
pub struct SpecMethodError {
    name: String,
    desc: String,
}

/// The Method type as defined in the JSON spec
#[derive(Debug, Serialize, Deserialize)]
pub struct SpecMethod {
    pub group: String,
    pub desc: String,
    pub name: String,
    pub args: HashMap<String, SpecArgument>,
    pub errors: HashMap<String, String>,
    pub has_paging: Option<bool>,
    pub default_count: Option<u64>,
}
