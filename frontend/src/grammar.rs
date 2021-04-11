include!(concat!(env!("OUT_DIR"), "/grammar.rs"));

use crate::lexer::Token;

use crate::ast::*;
use crate::ast::nodes::*;

use helpers::{elvis, some_or};

use parsing::ruler::RepresentableToken;

fn create_todo(location: &str) -> Box<dyn Node> {
    Box::new(
        Text {
            value: "[todo:".to_owned() + location + "]"
        }
    )
}

fn handle_token(token: &Token) -> Box<dyn Node> {
    match token {
        Token::Number { value, base } => {
            Box::new(
                Number {
                    value: value.clone(),
                    base: *base,
                }
            )
        }
        Token::Text { value } => {
            Box::new(
                Text {
                    value: value.clone()
                }
            )
        }
        _ => {
            Box::new(
                Text {
                    value: some_or! { token.get_value() => "" }.to_owned()
                }
            )
        }
    }
}

fn handle_pass(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 1 {
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

    let mut extractor = Extractor::new(|it: &mut Number| {
        result += &it.value;
    });

    target.accept_simple_visitor(&mut extractor);

    if !result.is_empty() {
        return result;
    }

    let mut extractor = Extractor::new(|it: &mut Text| {
        result += &it.value;
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
                    Text {
                        value: extract_value(operator1) + &extract_value(operator2)
                    }
                ),
                righter: righter,
            }
        )
    } else {
        create_todo("binary_long")
    }
}

fn handle_text_create(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 1 {
        Box::new(
            Text {
                value: extract_value(pattern.remove(0))
            }
        )
    } else {
        create_todo("text_create")
    }
}

fn handle_text_append(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 2 {
        let old = pattern.remove(0);
        let new = pattern.remove(0);

        Box::new(
            Text {
                value: extract_value(old) + &extract_value(new)
            }
        )
    } else {
        create_todo("text_append")
    }
}

fn handle_text_part_substitution(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 3 {
        pattern.remove(1)
    } else {
        create_todo("text_part_substitution")
    }
}

fn handle_text_parts_create(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 1 {
        Box::new(
            TextParts {
                parts: vec![pattern.remove(0)]
            }
        )
    } else {
        create_todo("text_parts_create")
    }
}

fn handle_text_parts_append(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 2 {
        let mut parts = pattern.remove(0);
        let element = pattern.remove(0);

        let mut extractor = Extractor::new(move |it: &mut TextParts| {
            it.parts.push(element);
        });

        parts.accept_simple_visitor(&mut extractor);
        parts
    } else {
        create_todo("text_parts_append")
    }
}

fn handle_string_pass_middle(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 3 {
        pattern.remove(1)
    } else {
        create_todo("string_pass_middle")
    }
}

fn handle_closure_arguments_append(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 3 {
        let mut list = pattern.remove(0);
        let element = pattern.remove(1); // skipping the operator

        let mut extractor = Extractor::new(move |it: &mut ClosureArguments| {
            it.values.push(element);
        });

        list.accept_simple_visitor(&mut extractor);
        list
    } else {
        create_todo("closure_arguments_append")
    }
}

fn handle_closure_arguments_create(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 1 {
        Box::new(
            ClosureArguments {
                values: vec![pattern.remove(0)]
            }
        )
    } else {
        create_todo("closure_arguments_create")
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
                    ClosureArguments {
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

macro_rules! check_node {
    ( $target:expr, $T:ty ) => {
        {
            let mut good = false;

            let mut extractor = Extractor::new(|_it: &mut $T| {
                good = true;
            });

            $target.accept_simple_visitor(&mut extractor);
            good
        }
    };
}

fn handle_leaf_number_append(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 2 {
        let mut maybe_parts = pattern.remove(0);
        let number = pattern.remove(0);

        if check_node!(maybe_parts, TextParts) {
            let mut extractor = Extractor::new(move |it: &mut TextParts| {
                it.parts.push(number);
            });

            maybe_parts.accept_simple_visitor(&mut extractor);
            return maybe_parts;
        }

        Box::new(
            TextParts {
                parts: vec![maybe_parts, number]
            }
        )
    } else {
        create_todo("leaf_number_append")
    }
}

fn handle_leaf_string_append(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 2 {
        let mut maybe_parts1 = pattern.remove(0);
        let mut maybe_parts2 = pattern.remove(0);

        if check_node!(maybe_parts1, TextParts) {
            if check_node!(maybe_parts2, TextParts) {
                let mut extractor1 = Extractor::new(move |it: &mut TextParts| {
                    let mut extractor2 = Extractor::new(move |that: &mut TextParts| {
                        let parts = std::mem::replace(&mut that.parts, vec![]);

                        for part in parts {
                            it.parts.push(part);
                        }
                    });

                    maybe_parts2.accept_simple_visitor(&mut extractor2);
                });

                maybe_parts1.accept_simple_visitor(&mut extractor1);
                return maybe_parts1;
            }

            let mut extractor1 = Extractor::new(move |it: &mut TextParts| {
                it.parts.push(maybe_parts2);
            });

            maybe_parts1.accept_simple_visitor(&mut extractor1);
            return maybe_parts1;
        }

        if check_node!(maybe_parts2, TextParts) {
            let mut extractor2 = Extractor::new(move |that: &mut TextParts| {
                that.parts.push(maybe_parts1);
            });

            maybe_parts2.accept_simple_visitor(&mut extractor2);
            return maybe_parts2;
        }

        Box::new(
            TextParts {
                parts: vec![maybe_parts1, maybe_parts2],
            }
        )
    } else {
        create_todo("leaf_number_append")
    }
}

fn handle_leaf_substitution_append(mut pattern: Vec<Box<dyn Node>>) -> Box<dyn Node> {
    if pattern.len() == 4 {
        let mut maybe_parts = pattern.remove(0);
        let inner = pattern.remove(1);

        if check_node!(maybe_parts, TextParts) {
            let mut extractor = Extractor::new(move |it: &mut TextParts| {
                it.parts.push(inner);
            });

            maybe_parts.accept_simple_visitor(&mut extractor);
            return maybe_parts;
        }

        Box::new(
            TextParts {
                parts: vec![maybe_parts, inner]
            }
        )
    } else {
        create_todo("leaf_substitution_append")
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
