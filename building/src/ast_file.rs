pub mod parsing;

use parsing::*;

use serde_json::Value;

use degen::rendering::{render, render_non_empty};
use degen::rust::{render_struct, render_mod, render_impl, render_struct_declaration_only};
use degen::rust::visitors::{render_node, render_trait_visitor, render_trait_visitor_no_body, render_impl_node, ACCEPTS, NodeInfo};

use inflector::Inflector;

fn render_nodes(ast_file: &ASTFile) -> String {
    let mut structs = vec![];

    for it in &ast_file.nodes {
        // let derives = render_derive(&["Clone"], 4);
        // let node = render_struct(&it.name, &it.fields, 4);
        // structs.push(derives + "\n" + &node);
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
            } else if that.proto == "Option<Box<dyn crate::ast::Node>>" {
                commands.push(render_print_option_node_field(&that.name, 0));
            } else if that.proto == "Box<dyn crate::ast::Node>" {
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

const EXTRACTOR: &'static str = "
    pub struct Extractor<T, F: FnOnce(&mut T) -> ()> {
        pub action: Option<F>,
        pub good: bool,
        pub junk: Option<T>,
    }

    impl <T, F: FnOnce(&mut T) -> ()> Extractor<T, F> {
        pub fn new(
            action: F
        ) -> Extractor<T, F> {
            Extractor {
                action: Some(action),
                good: false,
                junk: None,
            }
        }
    }
";

fn render_extractor_struct_and_impl() -> String {
    return render(EXTRACTOR, 0, &[]);
}

const CONCRETE_EXTRACTOR: &'static str = "
    impl <F: FnOnce(&mut $$) -> ()> SimpleVisitor for Extractor<$$, F> {
    $$
    }
";

const EXTRACTION_CALL: &'static str = "
    let that = std::mem::replace(&mut self.action, None);

    if let Some(action) = that {
        (action)(it);
        self.good = true;
    }
";

fn render_extractor_for(node: &NodeInfo) -> String {
    let snake = node.name.to_snake_case();
    let node_name = "nodes::".to_owned() + &node.name;
    let contents = render(EXTRACTION_CALL, 4, &[]);
    let visit = render(&VISIT_TEMPLATE_WITH_IT, 4, &[&snake, &node_name, "", "", &contents]);
    return render(CONCRETE_EXTRACTOR, 0, &[&node_name, &node_name, &visit]);
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
        if it.returns.is_empty() || !it.default.is_empty() {
            let visitor = render_trait_visitor(&it, &ast_file.nodes);
            pieces.push(visitor);
        }

        let visitor_no_body = render_trait_visitor_no_body(&it, &ast_file.nodes);
        pieces.push(visitor_no_body);
    }

    pieces.push(render_struct_declaration_only("ASTPrinter", 0));
    pieces.push(render_ast_printer(&ast_file));
    pieces.push(render_extractor_struct_and_impl());

    for it in &ast_file.nodes {
        let extractor = render_extractor_for(&it);
        pieces.push(extractor);
    }

    return pieces.join("\n\n");
}
