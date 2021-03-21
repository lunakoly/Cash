pub mod tokenization;

include!(concat!(env!("OUT_DIR"), "/ast.rs"));

use nodes::*;

use parsing::stream::analyzable_stream::{AnalyzableStream};

use tokenization::{Token, base_to_suffix};

pub fn parse<'a>(
    stream: &'a mut (dyn AnalyzableStream + 'a)
) -> File {
    let tokens = tokenization::process_input(stream);

    let leafs: Vec<Box<dyn Node>> = tokens.iter()
        .map(|it| {
            Box::new(
                Leaf {
                    value: match it {
                        Token::Operator { value } => value.clone(),
                        Token::NumberSegment { value, base } => value.clone() + &base_to_suffix(*base) + "[segment]",
                        Token::Number { value, base } => value.clone() + &base_to_suffix(*base),
                        Token::String { value } => value.clone(),
                        Token::Whitespace { value: _ } => "<whitespace>".to_owned(),
                        Token::Newline => "<newline>".to_owned(),
                    },
                }
            ) as Box<dyn Node>
        })
        .collect();

    return File {
        declarations: leafs,
    };
}
