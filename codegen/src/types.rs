use std::cmp::Ordering;
use std::collections::HashSet;
use SpecMethod;
use inflector::Inflector;

/// The Argument type used for generating the slack client
#[derive(Default, Debug, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub struct Argument {
    pub name: String,
    pub _type: String,
    pub example: String,
    pub desc: String,
    pub required: bool,
}

impl Argument {
    fn new(name: String, _type: String, example: String, desc: String, required: bool) -> Argument {
        Argument {
            name: name,
            _type: _type,
            example: example,
            desc: desc.lines().collect::<Vec<&str>>().join(". "),
            required: required,
        }
    }

    fn is_required(&self) -> bool {
        self.required
    }
}

impl Ord for Argument {
    fn cmp(&self, other: &Argument) -> Ordering {
        self.name.cmp(&other.name)
    }
}

/// The Method type used for generating the slack client
#[derive(Default, Debug, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub struct Method {
    pub name: String,
    pub full_name: String,
    pub endpoint: String,
    pub title: String,
    pub group: String,
    pub desc: Vec<String>,
    pub required_args: Vec<Argument>,
    // Serialized list of required arguments with type (used for constructor)
    pub required_args_types_list: Vec<String>,
    pub trait_bounds: String,
    pub optional_args: Vec<Argument>,
    // errors: HashMap<String, String>,
    pub has_paging: Option<bool>,
    pub default_count: Option<u64>,
}

impl Ord for Method {
    fn cmp(&self, other: &Method) -> Ordering {
        self.full_name.cmp(&other.full_name)
    }
}


impl From<SpecMethod> for Method {
    fn from(sm: SpecMethod) -> Method {

        // e.g. im.history -> ["im", "history"]
        let v: Vec<&str> = sm.name.splitn(2, ".").collect();
        let name = v[1].replace(".", "_");

        let mut required_args = Vec::new();
        let mut optional_args = Vec::new();
        let mut required_args_types_list = Vec::new();
        let mut trait_bounds_set = HashSet::new();

        for (name, arg) in sm.args {
            let _type = guess_type(&arg.example);
            let arg = Argument::new(name.clone(),
                                    _type.clone(),
                                    arg.example,
                                    arg.desc,
                                    arg.required);
            match arg.is_required() {
                true => {
                    if &arg._type == "String" {
                        // Support Into<String> conversion
                        // The function signature looks like this:
                        // function myfunc<S: Into<String>>(bla: S) -> ...
                        // Therefore we change the argument type to `S`.
                        trait_bounds_set.insert("S: Into<String>");
                        required_args_types_list.push(format!("{}: {}", name, "S".to_string()));
                    } else {
                        required_args_types_list.push(format!("{}: {}", name, _type));
                    }
                    required_args.push(arg);
                }
                false => optional_args.push(arg),
            }
        }

        required_args.sort();
        required_args_types_list.sort();
        optional_args.sort();

        let trait_bounds_vec: Vec<&str> = trait_bounds_set.into_iter().collect();
        let trait_bounds = format!("<{}>", trait_bounds_vec.join(", ")).to_string();

        Method {
            name: name.to_snake_case(),
            full_name: sm.name.clone().to_snake_case().replace(".", ""),
            endpoint: sm.name.clone(),
            title: name.to_title_case().split_whitespace().collect(),
            group: sm.group,
            desc: sm.desc.lines().map(String::from).collect(),
            has_paging: sm.has_paging,
            default_count: sm.default_count,
            required_args: required_args,
            required_args_types_list: required_args_types_list,
            trait_bounds: trait_bounds,
            optional_args: optional_args,
        }
    }
}

/// The Group type used for generating the slack client
#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    pub methods: Vec<Method>,
}

impl Group {
    pub fn new(name: String) -> Group {
        Group {
            name: name,
            methods: Vec::new(),
        }
    }

    pub fn add_method(&mut self, method: Method) -> &Group {
        self.methods.push(method);

        // Restore internal ordering of method names.
        // This is helpful to reduce the noise level during code generation:
        // Between two code generation runs, the order in which methods
        // were added doesn't matter.
        self.methods.sort();
        self
    }
}

fn guess_type(var: &str) -> String {
    if let Ok(i) = var.to_string().parse::<i64>() {
        match i {
            // Special case: Interpret "1" as bool
            1 => return "bool".to_string(),
            _ => return "i64".to_string(),
        };
    }
    if let Ok(_) = var.to_string().parse::<f64>() {
        return "f64".to_string();
    }
    match var {
        "true" | "false" => "bool".to_string(),
        _ => "String".to_string(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_type() {
        assert_eq!(guess_type(" "), "String");
        assert_eq!(guess_type("hello"), "String");
        assert_eq!(guess_type("true"), "bool");
        assert_eq!(guess_type("false"), "bool");
        assert_eq!(guess_type("1"), "bool");
        assert_eq!(guess_type("123"), "i64");
        assert_eq!(guess_type("123.0"), "f64");
    }
}