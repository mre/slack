// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate tera;
#[macro_use]

extern crate lazy_static;

extern crate inflector;
extern crate serde_json;
extern crate walkdir;

use std::path::Path;
use std::io::Write;
use std::fs;
use std::fs::File;

use tera::{Tera, Context};

use std::collections::HashMap;
mod schema;
mod spec_types;
use spec_types::SpecMethod;

mod types;
use types::{Method, Group};

mod errors;
use errors::*;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = compile_templates!("codegen/templates/**/*");
        tera
    };
}

/// generate triggers code generation and provides error handling.
pub fn generate(out_dir: &Path) {
    if let Err(ref e) = run(&out_dir.clone()) {
        use ::std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated.
        // Enable with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

/// Main entrypoint for code generation.
/// Reads the specification files, creates Rust types and writes the generated Rust code.
fn run(out_dir: &Path) -> Result<()> {
    // Parse raw JSON data from spec
    let mut spec_methods = Vec::new();
    for (schema, content) in schema::get_methods() {
        println!("Parsing {:?}", schema);
        let m: SpecMethod = serde_json::from_str(&content)?;
        spec_methods.push(m);
    }

    // Convert to internal types used
    // for generating the Slack client.
    let mut groups: HashMap<String, Group> = HashMap::new();
    let mut errors: HashMap<String, String> = HashMap::new();
    for spec_method in spec_methods {
        let group_name = spec_method.group.clone();
        let mut group = groups.entry(group_name.clone()).or_insert(Group::new(group_name));
        errors.extend(spec_method.errors.clone());
        group.add_method(Method::from(spec_method));
    }

    // Write auto-generated Slack types
    let dest_dir = Path::new(&out_dir);
    fs::create_dir_all(dest_dir)?;
    let _ = write_types(&out_dir, &groups);

    // Write auto-generated code for each Slack group
    for (group_name, group) in groups {
        let dest_dir = Path::new(&out_dir).join(&group_name);
        fs::create_dir_all(dest_dir)?;

        let _ = write_mod_rs(&out_dir, &group);

        for method in group.methods {
            let dest_path = Path::new(&out_dir)
                .join(&group_name)
                .join(format!("{}.rs", &method.name));

            println!("Writing to {:?}", dest_path);
            let mut f = File::create(&dest_path)?;

            let mut context = Context::new();
            context.add("method", &method);
            let content = TEMPLATES.render("method.rs", &context)?;

            f.write_all(content.as_bytes())?;
        }
    }
    Ok(())
}

// Every group will be written to a separate Rust module.
// Each module has its own mod.rs which reexports the methods
// and their MethodOption types. This makes using the client easier.
// We create the mod.rs here.
fn write_mod_rs(dir: &Path, group: &Group) -> Result<()> {
        let mod_rs_path = Path::new(&dir)
            .join(&group.name)
            .join("mod.rs");
        let mut mod_rs_file = File::create(&mod_rs_path)?;
        let mut context = Context::new();
        context.add("group", &group);
        let content = TEMPLATES.render("mod.rs", &context)?;
        let _ = mod_rs_file.write_all(content.as_bytes());
        Ok(())
}


// Auto-generated Slack types 
fn write_types(dir: &Path, groups: &HashMap<String, Group>) -> Result<()> {
        let slack_types_path = Path::new(&dir)
            .join("slack_types.rs");
        let mut slack_types_file = File::create(&slack_types_path)?;
        let mut context = Context::new();
        context.add("groups", &groups);
        let content = TEMPLATES.render("slack_types.rs", &context)?;
        let _ = slack_types_file.write_all(content.as_bytes());
        Ok(())
}

