use parsing::stream::*;

use crate::lexer::{Token};
use crate::liner::{Liner, base_to_suffix};

// use crate::value::*;
use crate::value::string::{StringValue};
// use crate::value::number::{NumberValue};

use crate::ast::*;
use crate::ast::nodes::*;

use std::rc::Rc;
use std::cell::RefCell;

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
}

impl <'a> Parser<'a> {
    fn parse(&mut self) {
        let mut leafs = vec![];

        for token in self.backend.grab() {
            leafs.push(
                Box::new(
                    Leaf {
                        value: match token {
                            Token::Operator { value } => Box::new(
                                StringValue::new(value.clone())
                            ),
                            Token::NumberSegment { value, base } => Box::new(
                                StringValue::new(
                                    value.clone() + &base_to_suffix(base) + "[segment]"
                                )
                            ),
                            Token::Number { value, base } => Box::new(
                                StringValue::new(
                                    value.clone() + &base_to_suffix(base)
                                )
                            ),
                            Token::String { value } => Box::new(
                                StringValue::new(value.clone())
                            ),
                            Token::Whitespace { value: _ } => Box::new(
                                StringValue::new("<whitespace>".to_owned())
                            ),
                            Token::Newline => Box::new(
                                StringValue::new("<newline>".to_owned())
                            ),
                            Token::End => Box::new(
                                StringValue::new("<end>".to_owned())
                            ),
                        }
                    }
                ) as Box<dyn Node>
            );
        }

        self.last_ast = Rc::new(
            RefCell::new(
                File {
                    declarations: leafs,
                }
            )
        );

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
            should_read: true
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
