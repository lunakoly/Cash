pub mod lexer;
pub mod liner;
pub mod value;

include!(concat!(env!("OUT_DIR"), "/ast.rs"));

use nodes::*;

use parsing::stream::*;

use crate::lexer::{Token};
use crate::liner::{base_to_suffix};

// use crate::value::*;
use crate::value::string::*;
// use crate::value::number::*;

pub fn parse<'a>(
    stream: &'a mut (dyn Stream<Vec<Token>> + 'a)
) -> File {
    let mut leafs = vec![];

    for token in stream.grab() {
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

    return File {
        declarations: leafs,
    };
}
