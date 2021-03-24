include!(concat!(env!("OUT_DIR"), "/grammar.rs"));

use crate::lexer::Token;

use crate::ast::*;
use crate::ast::nodes::*;

fn create_todo(location: &str) -> Box<dyn Node> {
    Box::new(
        Leaf {
            value: Token::String {
               value: "[todo:".to_owned() + location + "]"
            }
        }
    )
}

fn handle_pass(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if !pattern.is_empty() {
        pattern.remove(0)
    } else {
        create_todo("pass")
    }
}

fn handle_unary(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 2 {
        Box::new(
            Unary {
                operator: pattern.remove(0),
                target: pattern.remove(0),
            }
        )
    } else {
        create_todo("unary")
    }
}

fn handle_binary(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 3 {
        Box::new(
            Binary {
                lefter: pattern.remove(0),
                operator: pattern.remove(0),
                righter: pattern.remove(0),
            }
        )
    } else {
        create_todo("binary")
    }
}

fn handle_strings_append(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    // let mut list = pattern.remove(0);

    // let a = nodes::Extractor::<List>::new(|it| {

    // });

    // list.values.push(pattern.remove(1));

    create_todo("strings_append")
}

fn handle_strings_create(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 1 {
        Box::new(
            List {
                values: vec![pattern.remove(0)]
            }
        )
    } else {
        create_todo("strings_create")
    }
}

fn handle_leaf_single(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    create_todo("leaf_single")
}

fn handle_leaf_substitution(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    create_todo("leaf_substitution")
}

fn handle_leaf_closure_independent(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    create_todo("leaf_closure_independent")
}

fn handle_leaf_closure_dependent(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    create_todo("leaf_closure_dependent")
}

fn handle_command_append(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    create_todo("command_append")
}

fn handle_command_create(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    create_todo("command_create")
}

fn handle_pipeline_append(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    create_todo("pipeline_append")
}

fn handle_pipeline_create(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    create_todo("pipeline_create")
}

fn handle_expressions_append(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    create_todo("expressions_append")
}

fn handle_expressions_create(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    create_todo("expressions_create")
}
