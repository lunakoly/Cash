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

fn handle_token(token: &Token) -> Box<dyn Node> {
    Box::new(
        Leaf {
            value: token.clone()
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

fn extract_value(mut target: Box<dyn Node>) -> String {
    let mut result = String::new();

    let mut extractor = Extractor::new(|it: &mut Leaf| {
        result += match &it.value {
            Token::Operator { value } => &value,
            Token::Delimiter { value } => &value,
            Token::Number { value, .. } => &value,
            Token::String { value } => &value,
            _ => "",
        };
    });

    target.accept_simple_visitor(&mut extractor);
    return result;
}

fn handle_binary_long(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 4 {
        let lefter = pattern.remove(0);
        let operator1 = pattern.remove(0);
        let operator2 = pattern.remove(0);
        let righter = pattern.remove(0);

        Box::new(
            Binary {
                lefter: lefter,
                operator: Box::new(
                    Leaf {
                        value: Token::Operator {
                            value: extract_value(operator1) + &extract_value(operator2)
                        }
                    }
                ),
                righter: righter,
            }
        )
    } else {
        create_todo("binary_long")
    }
}

fn handle_strings_append(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 3 {
        let mut list = pattern.remove(0);
        let element = pattern.remove(1); // skipping the operator

        let mut extractor = Extractor::new(move |it: &mut List| {
            it.values.push(element);
        });

        list.accept_simple_visitor(&mut extractor);
        list
    } else {
        create_todo("strings_append")
    }
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
    if pattern.len() == 1 {
        pattern.remove(0)
    } else {
        create_todo("leaf_single")
    }
}

fn handle_leaf_substitution(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 3 {
        pattern.remove(1)
    } else {
        create_todo("leaf_substitution")
    }
}

fn handle_leaf_closure_independent(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 3 {
        Box::new(
            Closure {
                arguments: Box::new(
                    List {
                        values: vec![]
                    }
                ),
                body: pattern.remove(1)
            }
        )
    } else {
        create_todo("leaf_closure_independent")
    }
}

fn handle_leaf_closure_dependent(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 6 {
        Box::new(
            Closure {
                arguments: pattern.remove(1),
                body: pattern.remove(3) // skipping 1
            }
        )
    } else {
        create_todo("leaf_closure_dependent")
    }
}

fn handle_command_append(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 2 {
        let mut command = pattern.remove(0);
        let element = pattern.remove(0);

        let mut extractor = Extractor::new(move |it: &mut Command| {
            it.arguments.push(element);
        });

        command.accept_simple_visitor(&mut extractor);
        command
    } else {
        create_todo("command_append")
    }
}

fn handle_command_create(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 1 {
        Box::new(
            Command {
                arguments: vec![pattern.remove(0)]
            }
        )
    } else {
        create_todo("command_create")
    }
}

fn handle_pipeline_append(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 3 {
        let mut pipeline = pattern.remove(0);
        let element = pattern.remove(1); // skipping the operator

        let mut extractor = Extractor::new(move |it: &mut Pipeline| {
            it.commands.push(element);
        });

        pipeline.accept_simple_visitor(&mut extractor);
        pipeline
    } else {
        create_todo("pipeline_append")
    }
}

fn handle_pipeline_create(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 1 {
        Box::new(
            Pipeline {
                commands: vec![pattern.remove(0)]
            }
        )
    } else {
        create_todo("pipeline_create")
    }
}

fn handle_expressions_append(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 3 {
        let mut expressions = pattern.remove(0);
        let element = pattern.remove(1); // skipping the operator

        let mut extractor = Extractor::new(move |it: &mut Expressions| {
            it.values.push(element);
        });

        expressions.accept_simple_visitor(&mut extractor);
        expressions
    } else {
        create_todo("expressions_append")
    }
}

fn handle_expressions_create(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 1 {
        Box::new(
            Expressions {
                values: vec![pattern.remove(0)]
            }
        )
    } else {
        create_todo("expressions_create")
    }
}
