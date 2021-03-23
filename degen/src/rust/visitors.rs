use crate::rendering::*;
use crate::rust::*;

use inflector::Inflector;

pub struct NodeInfo {
    pub name: String,
    pub fields: Vec<FieldInfo>,
}

pub struct VisitorInfo {
    pub name: String,
    pub accepts: String,
    pub returns: String,
    pub default: String,
}

const ACCEPT_PROTO_TEMPLATE: &'static str = "
    fn accept_$$(&mut self, visitor: &mut dyn $$$$)$$;
";

const ACCEPT_TEMPLATE: &'static str = "
    fn accept_$$(&mut self, visitor: &mut dyn $$$$)$$ {
        $$visitor.visit_$$(self$$);
    }
";

pub const ACCEPTS: &'static str = ", data: $$";

const ACCEPTS_WITHOUT_USAGE: &'static str = ", _data: $$";

pub const RETURNS: &'static str = " -> $$";

fn render_accept_proto(visitor: &VisitorInfo) -> String {
    let snake = visitor.name.to_snake_case();
    let accepts = render_non_empty(ACCEPTS, &visitor.accepts);
    let returns = render_non_empty(RETURNS, &visitor.returns);
    return render(ACCEPT_PROTO_TEMPLATE, 4, &[&snake, &visitor.name, &accepts, &returns]);
}

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

pub fn render_node(visitors: &Vec<VisitorInfo>) -> String {
    let mut pieces = vec![];

    for it in visitors {
        pieces.push(render_accept_proto(it));
    }

    return render_trait("Node", &pieces.join("\n\n"), 0);
}

pub fn render_impl_node(node: &NodeInfo, visitors: &Vec<VisitorInfo>) -> String {
    let full_name = "nodes::".to_owned() + &node.name;
    let mut pieces = vec![];

    for it in visitors {
        pieces.push(render_accept(it, &node.name));
    }

    return render_impl("Node", &full_name, &pieces.join("\n\n"), 0);
}

const VISIT_TEMPLATE: &'static str = "
    fn visit_$$(&mut self, _it: &mut $$$$)$$ {
    $$
    }
";

pub fn render_trait_visitor(visitor: &VisitorInfo, nodes: &Vec<NodeInfo>) -> String {
    let mut pieces = vec!();

    for it in nodes {
        let snake = it.name.to_snake_case();
        let accepts = render_non_empty(ACCEPTS_WITHOUT_USAGE, &visitor.accepts);
        let returns = render_non_empty(RETURNS, &visitor.returns);
        let node_name = "nodes::".to_owned() + &it.name;
        let default = "    ".to_owned() + &visitor.default;
        let visit = render(VISIT_TEMPLATE, 4, &[&snake, &node_name, &accepts, &returns, &default]);
        pieces.push(visit);
    }

    return render_trait(&visitor.name, &pieces.join("\n\n"), 0);
}
