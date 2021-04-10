use std::env;
use std::fs;
use std::path::Path;
use std::ffi::OsString;
use serde_json::Value;

use building::{ast_file, grammar_file};

fn generate_ast_rs(out_dir: &OsString) {
    println!("cargo:rerun-if-changed=src/ast.json");

    let result_path = Path::new(&out_dir).join("ast.rs");

    let contents = fs::read_to_string("src/ast.json")
        .expect("reading the ast.json template");

    let template: Value = serde_json::from_str(&contents)
        .expect("parsing the ast.json contents");

    let source = ast_file::generation::ast_to_source(template);

    fs::write(&result_path, &source)
        .expect("writing the ast.js source file");
}

fn generate_grammar_rs(out_dir: &OsString) {
    println!("cargo:rerun-if-changed=src/grammar.json");

    let result_path = Path::new(&out_dir).join("grammar.rs");

    let contents = fs::read_to_string("src/grammar.json")
        .expect("reading the grammar.json template");

    let template: Value = serde_json::from_str(&contents)
        .expect("parsing the grammar.json contents");

    let source = grammar_file::generation::ast_to_source(template);

    fs::write(&result_path, &source)
        .expect("writing the grammar.js source file");
}

fn main() {
    let out_dir = env::var_os("OUT_DIR")
        .expect("reading the OUT_DIR environment variable");

    generate_ast_rs(&out_dir);
    generate_grammar_rs(&out_dir);
}
