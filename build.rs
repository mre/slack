extern crate codegen;

use std::env;
use std::path::Path;
use std::fs;

static CODEGEN_OUTPUT_DIR: &'static str = "src";

fn main() {

    let build_env = env::var_os("OUT_DIR").expect("OUT_DIR not specified");
    let build_dir = Path::new(&build_env).to_owned();
    let src_dir = Path::new(&CODEGEN_OUTPUT_DIR).to_owned();

    println!("Starting code generation from Slack API");
    codegen::generate(&build_dir);

    println!("Writing generated code to slack client src root");
    let _ = copy_dir(&build_dir, &src_dir);
}


// Recursive file copy. Adapted from
// https://gist.github.com/tubo028/aa83492e65850b2a9e07090441251d78
fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    if src.is_dir() {
        for entry in fs::read_dir(src)? {
            let src = entry?.path();
            let dst = &dst.join(src.file_name().unwrap());
            if src.is_dir() {
                println!("create_dir");
                let _ = fs::create_dir(&dst);
                println!("copy_dir");
                copy_dir(&src, &dst)?;
            } else {
                println!("copy {:?} to {:?}", &src, &dst);
                fs::copy(&src, &dst).unwrap();
            }
        }
    }
    Ok(())
}