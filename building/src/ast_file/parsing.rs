use serde_json::Value;

use degen::rust::FieldInfo;
use degen::rust::visitors::{NodeInfo, VisitorInfo};

pub struct ASTFile {
    pub nodes: Vec<NodeInfo>,
    pub visitors: Vec<VisitorInfo>,
}

fn parse_node_fields(node_body_json: &Value) -> Vec<FieldInfo> {
    let mut fields = vec!();

    for (field, proto) in node_body_json.as_object().unwrap() {
        if !field.starts_with('@') {
            fields.push(FieldInfo {
                name: field.clone(),
                proto: proto.as_str().unwrap().to_owned()
            });
        }
    }

    return fields;
}

fn parse_node_visualization(
    pattern: &str,
    fields: &Vec<FieldInfo>
) -> String {
    let mut iterator = fields.iter();
    let mut result = "\tlet mut result = String::new();\n\tresult += \"".to_owned();

    for symbol in pattern.chars() {
        match symbol {
            '%' => {
                result += "\";\n\tresult += &self.";
                result += &iterator.next().unwrap().name;
                result += ";\n\tresult += \"";
            }
            '$' => {
                result += "\";\n\tresult += &self.";
                result += &iterator.next().unwrap().name;
                result += ".visualize();\n\tresult += \"";
            }
            _ => {
                result.push(symbol);
            }
        }
    }

    result += "\";\n\treturn result;";
    return result;
}

fn parse_nodes(nodes_json: &Value) -> Vec<NodeInfo> {
    let mut nodes = vec!();

    for (node, node_body) in nodes_json.as_object().unwrap() {
        let fields = parse_node_fields(node_body);
        let visualization = parse_node_visualization(node_body["@visualize"].as_str().unwrap(), &fields);

        nodes.push(NodeInfo {
            name: node.clone(),
            fields: fields,
            visualization: visualization.clone()
        });
    }

    return nodes;
}

fn parse_visitors(visitors_json: &Value) -> Vec<VisitorInfo> {
    let mut visitors = vec!();

    for (visitor, visitor_body) in visitors_json.as_object().unwrap() {
        visitors.push(VisitorInfo {
            name: visitor.clone(),
            accepts: visitor_body["accepts"].as_str().unwrap().to_owned(),
            returns: visitor_body["returns"].as_str().unwrap().to_owned(),
            default: visitor_body["default"].as_str().unwrap().to_owned()
        });
    }

    return visitors;
}

fn enhance_fields_types(ast_file: &mut ASTFile) {
    let node_names: Vec<String> = ast_file.nodes.iter()
        .map(|it| it.name.clone())
        .collect();

    for node in &mut ast_file.nodes {
        for field in &mut node.fields {
            if field.proto == "Box<dyn Node>" {
                field.proto = "Box<dyn crate::cherry::Node>".to_owned();
            } else if node_names.contains(&field.proto) {
                field.proto = "crate::cherry::".to_owned() + &field.proto;
            }
        }
    }
}

pub fn parse_ast(template: Value) -> ASTFile {
    let mut ast_file = ASTFile {
        nodes: parse_nodes(&template["nodes"]),
        visitors: parse_visitors(&template["visitors"]),
    };

    enhance_fields_types(&mut ast_file);
    return ast_file;
}
