pub mod parsing;

use parsing::*;

use serde_json::Value;

use degen::rust::{render_struct, render_mod};
use degen::rust::visitors::{render_node, render_trait_visitor, render_impl_node};

pub fn render_nodes(ast_file: &ASTFile) -> String {
    let mut structs = vec![];

    for it in &ast_file.nodes {
        let node = render_struct(&it.name, &it.fields, 4);
        structs.push(node);
    }

    return render_mod("nodes", &structs.join("\n\n"), 0);
}

/// Parses the template as the ast.json file
/// and renders the corresponding rust source code.
pub fn ast_to_source(template: Value) -> String {
    let ast_file = parse_ast(template);

    let mut pieces = vec![
        "// THIS CODE IS AUTO-GENERATED".to_owned(),
        render_node(&ast_file.visitors),
        render_nodes(&ast_file)
    ];

    for it in &ast_file.nodes {
        let node = render_impl_node(&it, &ast_file.visitors);
        pieces.push(node);
    }

    for it in &ast_file.visitors {
        let visitor = render_trait_visitor(&it, &ast_file.nodes);
        pieces.push(visitor);
    }

    return pieces.join("\n\n");
}
