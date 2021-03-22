pub mod lexer;
pub mod liner;

include!(concat!(env!("OUT_DIR"), "/ast.rs"));

use nodes::*;

use parsing::stream::*;

use crate::lexer::{Token};
use crate::liner::{base_to_suffix};

pub fn parse<'a>(
    stream: &'a mut (dyn Stream<Vec<Token>> + 'a)
) -> File {
    let mut leafs = vec![];

    for token in stream.grab() {
        leafs.push(
            Box::new(
                Leaf {
                    value: match token {
                        Token::Operator { value } => value.clone(),
                        Token::NumberSegment { value, base } => value.clone() + &base_to_suffix(base) + "[segment]",
                        Token::Number { value, base } => value.clone() + &base_to_suffix(base),
                        Token::String { value } => value.clone(),
                        Token::Whitespace { value: _ } => "<whitespace>".to_owned(),
                        Token::Newline => "<newline>".to_owned(),
                        Token::End => "<end>".to_owned(),
                    }
                }
            ) as Box<dyn Node>
        );
    }

    return File {
        declarations: leafs,
    };
}
