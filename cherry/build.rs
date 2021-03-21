use std::env;
use std::fs;
use std::path::Path;
use std::ffi::OsString;
use serde_json::Value;

use building::ast_file::ast_to_source;

fn generate_ast_rs(out_dir: &OsString) {
    println!("cargo:rerun-if-changed=src/ast.json");

    let result_path = Path::new(&out_dir).join("ast.rs");

    let contents = fs::read_to_string("src/ast.json")
        .expect("reading the ast.json template");

    let template: Value = serde_json::from_str(&contents)
        .expect("parsing the ast.json contents");

    let source = ast_to_source(template);

    fs::write(&result_path, &source)
        .expect("writing the ast.js source file");
}

fn main() {
    let out_dir = env::var_os("OUT_DIR")
        .expect("reading the OUT_DIR environment variable");

    generate_ast_rs(&out_dir);
}
