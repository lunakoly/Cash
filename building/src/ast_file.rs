pub mod parsing;

use parsing::*;

use serde_json::Value;

use degen::rendering::{render, render_non_empty};
use degen::rust::{render_struct, render_mod, render_impl, render_struct_declaration_only};
use degen::rust::visitors::{render_node, render_trait_visitor, render_impl_node, ACCEPTS};

use inflector::Inflector;

fn render_nodes(ast_file: &ASTFile) -> String {
    let mut structs = vec![];

    for it in &ast_file.nodes {
        let node = render_struct(&it.name, &it.fields, 4);
        structs.push(node);
    }

    return render_mod("nodes", &structs.join("\n\n"), 0);
}

/// This differs from VISIT_TEMPLATE in
/// a way that it doesn't have `_` before `it`
const VISIT_TEMPLATE_WITH_IT: &'static str = "
    fn visit_$$(&mut self, it: &mut $$$$)$$ {
    $$
    }
";

const PRINTING: &'static str = "
    println!(\"$$ {{\");

    $$

    println!(\"{}}}\", \" \".repeat(data));
";

const PRINT_SIMPLE_FIELD: &'static str = "
    print!(\"{}$$ = \", \" \".repeat(data + 2));
    it.$$.accept_leveled_visitor(self, data + 2);
";

fn render_print_simple_field(field: &str, indent: usize) -> String {
    return render(PRINT_SIMPLE_FIELD, indent, &[field, field])
}

const PRINT_LIST_FIELD: &'static str = "
    for that in 0..it.$$.len() {
        print!(\"{}$$.{} = \", \" \".repeat(data + 2), that);
        it.$$[that].accept_leveled_visitor(self, data + 2);
    }
";

fn render_print_list_field(field: &str, indent: usize) -> String {
    return render(PRINT_LIST_FIELD, indent, &[field, field, field])
}

const PRINT_DEBUG_FIELD: &'static str = "
    println!(\"{}$$ = {:?}\", \" \".repeat(data + 2), &it.$$);
";

fn render_print_debug_field(field: &str, indent: usize) -> String {
    return render(PRINT_DEBUG_FIELD, indent, &[field, field])
}

const PRINT_OPTION_NODE_FIELD: &'static str = "
    if let Some(that) = &mut it.$$ {
        print!(\"{}$$ = \", \" \".repeat(data + 2));
        that.accept_leveled_visitor(self, data + 2);
    }
";

fn render_print_option_node_field(field: &str, indent: usize) -> String {
    return render(PRINT_OPTION_NODE_FIELD, indent, &[field, field])
}

fn render_ast_printer(ast_file: &ASTFile) -> String {
    let mut methods = vec![];

    for it in &ast_file.nodes {
        let snake = it.name.to_snake_case();
        let accepts = render_non_empty(ACCEPTS, "usize");
        let node_name = "nodes::".to_owned() + &it.name;

        let mut commands = vec![];

        for that in &it.fields {
            if that.proto.starts_with("Vec") {
                commands.push(render_print_list_field(&that.name, 0));
            } else if that.proto == "Option<Box<dyn crate::Node>>" {
                commands.push(render_print_option_node_field(&that.name, 0));
            } else if that.proto == "Box<dyn crate::Node>" {
                commands.push(render_print_simple_field(&that.name, 0));
            } else {
                commands.push(render_print_debug_field(&that.name, 0));
            }
        }

        let contents = render(PRINTING, 4, &[&it.name, &commands.join("\n\n")]);
        let visit = render(&VISIT_TEMPLATE_WITH_IT, 4, &[&snake, &node_name, &accepts, "", &contents]);

        methods.push(visit);
    }

    return render_impl("LeveledVisitor", "ASTPrinter", &methods.join("\n\n"), 0);
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

    pieces.push(render_struct_declaration_only("ASTPrinter", 0));

    pieces.push(render_ast_printer(&ast_file));
    return pieces.join("\n\n");
}
