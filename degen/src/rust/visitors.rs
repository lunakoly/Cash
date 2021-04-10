use crate::rendering::*;
use crate::rust::*;

use inflector::Inflector;

/// Represents a visitable node struct.
pub struct NodeInfo {
    pub name: String,
    pub fields: Vec<FieldInfo>,
}

/// Represents a visitor.
pub struct VisitorInfo {
    pub name: String,
    pub accepts: String,
    pub returns: String,
    pub default: String,
}

/// Template for an `accept` function
/// without a body.
const ACCEPT_PROTO_TEMPLATE: &'static str = "
    fn accept_$$(&mut self, visitor: &mut dyn $$$$)$$;
";

/// Template for an `accept` function
/// with a body that delegates to a
/// `visit` call of the `visitor` argument.
const ACCEPT_TEMPLATE: &'static str = "
    fn accept_$$(&mut self, visitor: &mut dyn $$$$)$$ {
        $$visitor.visit_$$(self$$);
    }
";

/// Optional data passed as the `data`
/// parameter during the visiting process.
pub const ACCEPTS: &'static str = ", data: $$";

/// Optional data passed as the `data`
/// parameter during the visiting process,
/// but the parameter has a `_` prefix.
const ACCEPTS_WITHOUT_USAGE: &'static str = ", _data: $$";

/// Optional data returned from each visiting
/// function.
pub const RETURNS: &'static str = " -> $$";

/// Renders an `accept` prototype declaraton
/// for the given visitor.
fn render_accept_proto(visitor: &VisitorInfo) -> String {
    let snake = visitor.name.to_snake_case();
    let accepts = render_non_empty(ACCEPTS, &visitor.accepts);
    let returns = render_non_empty(RETURNS, &visitor.returns);
    return render(ACCEPT_PROTO_TEMPLATE, 4, &[&snake, &visitor.name, &accepts, &returns]);
}

/// Renders an `accept` implementation
/// for the given visitor, and
/// the name of the node.
fn render_accept(
    visitor: &VisitorInfo,
    self_name: &str
) -> String {
    let visitor_snake = visitor.name.to_snake_case();
    let self_snake = self_name.to_snake_case();

    let return_keyword = if !visitor.returns.is_empty() {
        "return "
    } else {
        ""
    };

    let data_parameter = if !visitor.accepts.is_empty() {
        ", data"
    } else {
        ""
    };

    let accepts = render_non_empty(ACCEPTS, &visitor.accepts);
    let returns = render_non_empty(RETURNS, &visitor.returns);

    return render(ACCEPT_TEMPLATE, 4, &[
        &visitor_snake, &visitor.name, &accepts, &returns, &return_keyword, &self_snake, &data_parameter
    ]);
}

/// Renders the 'Node' trait that
/// every other node must implement. It
/// contains declarations of all the required
/// `accept` functions.
pub fn render_node(visitors: &Vec<VisitorInfo>) -> String {
    let mut pieces = vec![];

    for it in visitors {
        if it.returns.is_empty() || !it.default.is_empty() {
            pieces.push(render_accept_proto(it));
        }
    }

    return render_trait("Node", &pieces.join("\n\n"), 0);
}

/// Renders an impl for a particular node
/// struct.
pub fn render_impl_node(node: &NodeInfo, visitors: &Vec<VisitorInfo>) -> String {
    let full_name = "nodes::".to_owned() + &node.name;
    let mut pieces = vec![];

    for it in visitors {
        if it.returns.is_empty() || !it.default.is_empty() {
            pieces.push(render_accept(it, &node.name));
        }
    }

    return render_impl("Node", &full_name, &pieces.join("\n\n"), 0);
}

/// Template for a `visit` fucntion declaration.
const VISIT_TEMPLATE: &'static str = "
    fn visit_$$(&mut self, it: &mut $$$$)$$;
";

/// Template for a visit function implementation
/// with some body.
const VISIT_TEMPLATE_DEFAULT: &'static str = "
    fn visit_$$(&mut self, _it: &mut $$$$)$$ {
    $$
    }
";

/// Template for a visit function implementation
/// with a delegate call so some other trait's
/// implementation.
const VISIT_TEMPLATE_DELEGATE: &'static str = "
    fn visit_$$(&mut self, it: &mut $$$$)$$ {
        (self as &mut dyn $$).visit_$$(it$$)
    }
";

/// Renders a visitor trait with
/// `visit` functions without any body,
/// and an optional visitor trait with
/// a default body (if possible).
pub fn render_visitor_versions(visitor: &VisitorInfo, nodes: &Vec<NodeInfo>) -> String {
    let mut no_body_peices = vec!();
    let mut default_peices = vec!();
    let mut delegate_peices = vec!();

    let visitor_name_default = visitor.name.clone() + "Default";

    for it in nodes {
        let snake = it.name.to_snake_case();
        let accepts = render_non_empty(ACCEPTS, &visitor.accepts);
        let returns = render_non_empty(RETURNS, &visitor.returns);
        let node_name = "nodes::".to_owned() + &it.name;

        no_body_peices.push(
            render(VISIT_TEMPLATE, 4, &[
                &snake, &node_name, &accepts, &returns
            ])
        );

        if visitor.returns.is_empty() || !visitor.default.is_empty() {
            let accepts_without_usage = render_non_empty(ACCEPTS_WITHOUT_USAGE, &visitor.accepts);
            let default = "    ".to_owned() + &visitor.default;

            default_peices.push(
                render(VISIT_TEMPLATE_DEFAULT, 4, &[
                    &snake, &node_name, &accepts_without_usage, &returns, &default
                ])
            );

            let data_parameter = if !visitor.accepts.is_empty() {
                ", data"
            } else {
                ""
            };

            let accepts = render_non_empty(ACCEPTS, &visitor.accepts);

            delegate_peices.push(
                render(VISIT_TEMPLATE_DELEGATE, 4, &[
                    &snake, &node_name, &accepts, &returns, &visitor_name_default, &snake, &data_parameter
                ])
            );
        }
    }

    if default_peices.is_empty() {
        return render_trait(&visitor.name, &no_body_peices.join("\n\n"), 0);
    }

    let impl_name = "<U: ".to_owned() + &visitor_name_default + "> " + &visitor.name;

    return vec![
        render_trait(&visitor.name, &no_body_peices.join("\n\n"), 0),
        render_trait(&visitor_name_default, &default_peices.join("\n\n"), 0),
        render_impl(&impl_name, "U", &delegate_peices.join("\n\n"), 0),
    ].join("\n\n");
}

/// Renders structs for particular nodes
/// inside of a `nodes::` submodule.
fn render_nodes(nodes: &Vec<NodeInfo>) -> String {
    let mut structs = vec![];

    for it in nodes {
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

/// Template for printing 'wrapping'
/// curly braces around the node contents.
const PRINTING: &'static str = "
    println!(\"$$ {{\");

    $$

    println!(\"{}}}\", \" \".repeat(data));
";

/// Template for printing the value
/// of a field via the recurrent call to
/// `accept`.
const PRINT_SIMPLE_FIELD: &'static str = "
    print!(\"{}$$ = \", \" \".repeat(data + 2));
    it.$$.accept_leveled_visitor(self, data + 2);
";

/// Renders a simple field representation.
fn render_print_simple_field(field: &str, indent: usize) -> String {
    return render(PRINT_SIMPLE_FIELD, indent, &[field, field])
}

/// Template for printing lists. Elements
/// are printed via the recurrent `accept` call.
const PRINT_LIST_FIELD: &'static str = "
    for that in 0..it.$$.len() {
        print!(\"{}$$.{} = \", \" \".repeat(data + 2), that);
        it.$$[that].accept_leveled_visitor(self, data + 2);
    }
";

/// Renders a list representation.
fn render_print_list_field(field: &str, indent: usize) -> String {
    return render(PRINT_LIST_FIELD, indent, &[field, field, field])
}

/// Template for printing a field
/// value via the debug `{:?}` formatter.
const PRINT_DEBUG_FIELD: &'static str = "
    println!(\"{}$$ = {:?}\", \" \".repeat(data + 2), &it.$$);
";

/// Renders the field representation via
/// the debug `{:?}` formatter.
fn render_print_debug_field(field: &str, indent: usize) -> String {
    return render(PRINT_DEBUG_FIELD, indent, &[field, field])
}

/// Template for rendering an `Option<Node>`.
const PRINT_OPTION_NODE_FIELD: &'static str = "
    if let Some(that) = &mut it.$$ {
        print!(\"{}$$ = \", \" \".repeat(data + 2));
        that.accept_leveled_visitor(self, data + 2);
    }
";

/// Renders the representation of an `Option<Node>`
fn render_print_option_node_field(field: &str, indent: usize) -> String {
    return render(PRINT_OPTION_NODE_FIELD, indent, &[field, field])
}

/// Renders a `LeveledVisitor` impl for the
/// `ASTPrinter` struct.
fn render_ast_printer(nodes: &Vec<NodeInfo>) -> String {
    let mut methods = vec![];

    for it in nodes {
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

/// Template for a struct and a trait
/// for 'casting' a `Node` to a specific type.
const EXTRACTOR: &'static str = "
    pub struct Extractor<T, F: FnOnce(&mut T) -> ()> {
        junk: std::marker::PhantomData<T>,
        pub action: Option<F>,
    }

    impl <T, F: FnOnce(&mut T) -> ()> Extractor<T, F> {
        pub fn new(
            action: F
        ) -> Extractor<T, F> {
            Extractor {
                action: Some(action),
                junk: std::marker::PhantomData,
            }
        }
    }
";

/// Renders the extractor common code.
fn render_extractor_struct_and_impl() -> String {
    return render(EXTRACTOR, 0, &[]);
}

/// Template for a concrete extractor
/// (for a concrete Node).
const CONCRETE_EXTRACTOR: &'static str = "
    impl <F: FnOnce(&mut $$) -> ()> SimpleVisitorDefault for Extractor<$$, F> {
    $$
    }
";

/// Template for an action call within
/// an extractor.
const EXTRACTION_CALL: &'static str = "
    let that = std::mem::replace(&mut self.action, None);

    if let Some(action) = that {
        (action)(it);
    }
";

/// Renders an extractor for the node.
fn render_extractor_for(node: &NodeInfo) -> String {
    let snake = node.name.to_snake_case();
    let node_name = "nodes::".to_owned() + &node.name;
    let contents = render(EXTRACTION_CALL, 4, &[]);
    let visit = render(&VISIT_TEMPLATE_WITH_IT, 4, &[&snake, &node_name, "", "", &contents]);
    return render(CONCRETE_EXTRACTOR, 0, &[&node_name, &node_name, &visit, &node_name, &node_name]);
}

/// Renders nodes, visitors, and other
/// corresponding stuff.
pub fn render_ast(nodes: &Vec<NodeInfo>, visitors: &Vec<VisitorInfo>) -> String {
    let mut pieces = vec![
        "// THIS CODE IS AUTO-GENERATED".to_owned(),
        render_node(visitors),
        render_nodes(nodes)
    ];

    for it in nodes {
        let node = render_impl_node(&it, visitors);
        pieces.push(node);
    }

    for it in visitors {
        let visitors = render_visitor_versions(&it, nodes);
        pieces.push(visitors);
    }

    pieces.push(render_struct_no_body("ASTPrinter", 0));
    pieces.push(render_ast_printer(nodes));
    pieces.push(render_extractor_struct_and_impl());

    for it in nodes {
        let extractor = render_extractor_for(&it);
        pieces.push(extractor);
    }

    return pieces.join("\n\n");
}
