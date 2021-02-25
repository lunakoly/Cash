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
    let mut result = "\tlet mut result = String::new();\n\t\tresult += \"".to_owned();

    for symbol in pattern.chars() {
        match symbol {
            '%' => {
                result += "\";\n\t\tresult += &self.";
                result += &iterator.next().unwrap().name;
                result += ";\n\t\tresult += \"";
            }
            '$' => {
                result += "\";\n\t\tresult += &self.";
                result += &iterator.next().unwrap().name;
                result += ".visualize();\n\t\tresult += \"";
            }
            _ => {
                result.push(symbol);
            }
        }
    }

    result += "\";\n\t\treturn result;";
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

pub fn parse_ast(template: Value) -> ASTFile {
    return ASTFile {
        nodes: parse_nodes(&template["nodes"]),
        visitors: parse_visitors(&template["visitors"]),
    };
}
