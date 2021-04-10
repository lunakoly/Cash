use super::parsing::*;

use serde_json::Value;

use degen::rust::visitors::{render_ast};

/// Parses the template as the ast.json file
/// and renders the corresponding rust source code.
pub fn ast_to_source(template: Value) -> String {
    let ast_file = parse_ast(template);
    return render_ast(&ast_file.nodes, &ast_file.visitors);
}
