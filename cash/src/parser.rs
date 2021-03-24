use parsing::stream::*;

use crate::lexer::{Token};
use crate::liner::{Liner};

// use crate::value::*;
// use crate::value::string::{StringValue};
// use crate::value::number::{NumberValue};

use crate::ast::*;
use crate::ast::nodes::*;

use crate::grammar::{get_rules, Rule, Branch};

use std::rc::Rc;
use std::cell::RefCell;

fn is_same_type(token_type: &str, token: &Token) -> bool {
    match token {
        Token::Operator { .. } => token_type == "operator",
        Token::Number { .. } => token_type == "number",
        Token::String { .. } => token_type == "string",
        Token::Whitespace { .. } => token_type == "whitespace",
        Token::Newline { .. } => token_type == "newline",
        Token::End { .. } => token_type == "end",
        _ => false,
    }
}

fn get_token_value(token: &Token) -> Option<&str> {
    match token {
        Token::Operator { value } => Some(value),
        Token::Number { value, .. } => Some(value),
        Token::String { value } => Some(value),
        _ => None,
    }
}

fn get_rule_by_name<'a>(rules: &'a Vec<Rule>, name: &str) -> Option<&'a Rule<'a>> {
    for rule in rules {
        println!("Checking: {:?} == {:?}", rule.name, name);
        if rule.name == name {
            return Some(rule as &'a Rule);
        }
    }

    println!("Not found: {:?}", name);

    return None;
}

fn apply_item(
    rules: &Vec<Rule>,
    item: &str,
    tokens: &[Token],
    token_index: usize
) -> (Option<Box<dyn Node>>, usize) {
    if token_index >= tokens.len() {
        return (None, token_index);
    }

    if item.starts_with("@") {
        let rule_name = item.chars().skip(1).collect::<String>();
        return apply_rule(rules, &rule_name, tokens, token_index);
    }

    if item.starts_with("#") {
        let token_type = item.chars().skip(1).collect::<String>();

        if is_same_type(&token_type, &tokens[token_index]) {
            return (
                Some(
                    Box::new(
                        Leaf {
                            value: tokens[token_index].clone()
                        }
                    )
                ),
                token_index + 1
            );
        }

        return (None, token_index);
    }

    let token_value = get_token_value(&tokens[token_index]);

    if Some(item) == token_value {
        return (
            Some(
                Box::new(
                    Leaf {
                        value: tokens[token_index].clone()
                    }
                )
            ),
            token_index + 1
        );
    }

    println!("ITEM NOT FOUND: {:?} for {:?}", item, tokens[token_index]);
    return (None, token_index);
}

fn apply_branch(
    rules: &Vec<Rule>,
    branch: &Branch,
    pattern_item_index: usize,
    tokens: &[Token],
    token_index: usize,
) -> (Option<Vec<Box<dyn Node>>>, usize) {
    let mut moved_token_index = token_index;
    let mut values = vec![];

    for it in pattern_item_index..branch.pattern.len() {
        let (item, new_token_index) = apply_item(
            rules,
            branch.pattern[it],
            tokens,
            moved_token_index
        );

        if let Some(thing) = item {
            values.push(thing);
            moved_token_index = new_token_index;
        } else {
            return (None, token_index);
        }
    }

    return (Some(values), moved_token_index);
}

fn apply_simple_rule(
    rules: &Vec<Rule>,
    rule_name: &str,
    tokens: &[Token],
    token_index: usize,
) -> (Option<Box<dyn Node>>, usize) {
    if let Some(rule) = get_rule_by_name(rules, rule_name) {
        for branch in &rule.simple_branches {
            let (values, new_token_index) = apply_branch(
                rules,
                branch,
                0,
                tokens,
                token_index
            );

            if let Some(values) = values {
                return (Some((branch.handler)(values)), new_token_index);
            }
        }
    }

    println!("SIMPLE RULE NOT FOUND [{:?}]", rule_name);
    return (None, token_index);
}

fn apply_rule(
    rules: &Vec<Rule>,
    rule_name: &str,
    tokens: &[Token],
    token_index: usize,
) -> (Option<Box<dyn Node>>, usize) {
    let (mut result, mut moved_token_index) = apply_simple_rule(rules, rule_name, tokens, token_index);

    if let Some(rule) = get_rule_by_name(rules, rule_name) {
        let mut applied = true;

        while applied {
            applied = false;

            if let Some(mut thing) = result {
                for branch in &rule.recursive_branches {
                    let (maybe_values, new_token_index) = apply_branch(
                        rules,
                        branch,
                        1,
                        tokens,
                        moved_token_index
                    );

                    if let Some(mut values) = maybe_values {
                        values.push(thing);
                        thing = (branch.handler)(values);
                        moved_token_index = new_token_index;
                        applied = true;
                        break;
                    }
                }

                result = Some(thing);
            }
        }

        return (result, moved_token_index);
    }

    println!("RULE NOT FOUND [{:?}]", rule_name);
    return (None, token_index);
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        return self.declarations.is_empty() && other.declarations.is_empty();
    }
}

impl Eq for File {}

pub struct Parser<'a> {
    pub backend: Liner<'a>,
    pub last_ast: Rc<RefCell<File>>,
    pub should_read: bool,
    pub rules: Vec<Rule<'a>>,
}

impl <'a> Parser<'a> {
    fn parse(&mut self) {
        // let mut leafs = vec![];

        // for token in self.backend.grab() {
        //     leafs.push(
        //         Box::new(
        //             Leaf {
        //                 value: token
        //             }
        //         ) as Box<dyn Node>
        //     );
        // }

        // self.last_ast = Rc::new(
        //     RefCell::new(
        //         File {
        //             declarations: leafs,
        //         }
        //     )
        // );

        let tokens = self.backend.grab();
        let (ast, _) = apply_rule(
            &self.rules,
            "expression",
            &tokens,
            0
        );

        if let Some(thing) = ast {
            self.last_ast = Rc::new(
                RefCell::new(
                    File {
                        declarations: vec![thing]
                    }
                )
            );
        } else {
            self.last_ast = Rc::new(
                RefCell::new(
                    File {
                        declarations: vec![
                            Box::new(
                                Leaf {
                                    value: Token::String {
                                        value: "[error]".to_owned()
                                    }
                                }
                            )
                        ]
                    }
                )
            );
        }


        self.should_read = false;
    }

    pub fn new(
        backend: &'a mut (dyn Stream<Token> + 'a),
    ) -> Parser<'a> {
        return Parser::<'a> {
            backend: Liner::<'a>::new(backend),
            last_ast: Rc::new(
                RefCell::new(
                    File {
                        declarations: vec![]
                    }
                )
            ),
            should_read: true,
            rules: get_rules()
        };
    }
}

impl <'a> Stream<Rc<RefCell<File>>> for Parser<'a> {
    fn get_end_value(&self) -> Rc<RefCell<File>> {
        return Rc::new(
            RefCell::new(
                File {
                    declarations: vec![]
                }
            )
        );
    }

    fn peek(&mut self) -> Rc<RefCell<File>> {
        if self.should_read {
            self.parse();
        }

        return self.last_ast.clone();
    }

    fn step(&mut self) {
        if self.should_read {
            self.parse();
        }

        self.should_read = true;
    }

    fn get_offset(&self) -> usize {
        return self.backend.get_offset();
    }
}
