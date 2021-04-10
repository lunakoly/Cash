use parsing::stream::*;

use crate::lexer::{Token};

pub fn transform(
    tokens: &[Token],
    apply: &dyn Fn(&Token, &Token, &mut Vec<Token>) -> ()
) -> Vec<Token> {
    let mut result = vec![];
    let mut iterator = tokens.iter();

    if let Some(first) = iterator.next() {
        let mut last = (*first).clone();

        result.push(last.clone());

        for next in iterator {
            apply(&last, &next, &mut result);

            // in fact, this should always
            // succeed
            if let Some(value) = result.last() {
                last = value.clone();
            }
        }
    }

    println!("After Some Transformation:");
    for it in &result {
        println!("    {:?}", it);
    }
    println!("");

    return result;
}

fn transform_duplicate_whitespaces(
    last: &Token,
    next: &Token,
    tokens: &mut Vec<Token>
) {
    match (last, next) {
        (
            Token::Whitespace { value: last_value },
            Token::Whitespace { value: next_value }
        ) => {
            tokens.pop();
            tokens.push(
                Token::Whitespace { value: last_value.clone() + &next_value }
            );
        },
        _ => {
            tokens.push(next.clone());
        },
    }
}

pub fn base_to_suffix(base: u8) -> String {
    match base {
        2 => "b".to_owned(),
        8 => "o".to_owned(),
        16 => "h".to_owned(),
        _ => "".to_owned(),
    }
}

fn transform_numbers(
    last: &Token,
    next: &Token,
    tokens: &mut Vec<Token>
) {
    match (last, next) {
        (
            Token::NumberSegment { value: last_value, base: last_base },
            Token::NumberSegment { value: next_value, base: next_base }
        ) => {
            // it's only possible that the 2 tokens
            // have different base

            // special case: when the second number
            // is 'b'_16 that must be treated as
            // 'b' suffix of a binary number

            if next_value == "b" && *last_base == 2 {
                tokens.pop();
                return tokens.push(Token::Number {
                    value: last_value.clone(),
                    base: *last_base,
                });
            }

            tokens.pop();
            tokens.push(
                Token::NumberSegment {
                    value: last_value.clone() + &next_value,
                    base: std::cmp::max(*last_base, *next_base)
                }
            );
        },
        (
            Token::NumberSegment { value: last_value, base },
            Token::String { value: next_value }
        ) => {
            tokens.pop();

            if *next_value == "b" {
                if *base == 2 {
                    return tokens.push(Token::Number {
                        value: last_value.clone(),
                        base: 2
                    });
                }

                return tokens.push(
                    Token::String {
                        value: last_value.clone() + &next_value
                    }
                );
            }

            if *next_value == "o" {
                if *base <= 8 {
                    return tokens.push(Token::Number {
                        value: last_value.clone(),
                        base: 8
                    });
                }

                return tokens.push(
                    Token::String {
                        value: last_value.clone() + &next_value
                    }
                );
            }

            if *next_value == "h" {
                if *base <= 16 {
                    return tokens.push(Token::Number {
                        value: last_value.clone(),
                        base: 16
                    });
                }

                return tokens.push(
                    Token::String {
                        value: last_value.clone() + &next_value
                    }
                );
            }

            tokens.push(
                Token::String { value: last_value.clone() + &next_value }
            );
        },
        (
            Token::String { value: last_value },
            Token::NumberSegment { value: next_value, base: _ }
        ) => {
            tokens.pop();
            tokens.push(
                Token::String { value: last_value.clone() + &next_value }
            );
        },
        (
            Token::NumberSegment { value: last_value, base },
            _
        ) => {
            tokens.pop();

            if *base <= 10 {
                tokens.push(
                    Token::Number {
                        value: last_value.clone(),
                        base: 10,
                    }
                );
            } else {
                tokens.push(
                    Token::String { value: last_value.clone() }
                );
            }

            tokens.push(next.clone());
        },
        _ => {
            tokens.push(next.clone());
        },
    }
}

fn transform_tight_strings(
    last: &Token,
    next: &Token,
    tokens: &mut Vec<Token>
) {
    match (last, next) {
        (
            Token::Operator { value: last_value },
            Token::String { value: next_value }
        ) => {
            tokens.pop();
            tokens.push(
                Token::String { value: last_value.clone() + &next_value }
            );
        },
        (
            Token::String { value: last_value },
            Token::Operator { value: next_value }
        ) => {
            tokens.pop();
            tokens.push(
                Token::String { value: last_value.clone() + &next_value }
            );
        },
        (
            Token::String { value: last_value },
            Token::String { value: next_value }
        ) => {
            tokens.pop();
            tokens.push(
                Token::String { value: last_value.clone() + &next_value }
            );
        },
        (
            Token::Number { value: last_value, base: _ },
            Token::String { value: next_value }
        ) => {
            tokens.pop();
            tokens.push(
                Token::String { value: last_value.clone() + &next_value }
            );
        },
        (
            Token::String { value: last_value },
            Token::Number { value: next_value, base: _ }
        ) => {
            tokens.pop();
            tokens.push(
                Token::String { value: last_value.clone() + &next_value }
            );
        },
        _ => {
            tokens.push(next.clone());
        },
    }
}

pub struct Liner<'a> {
    pub backend: &'a mut (dyn Stream<Token> + 'a),
    pub end_token_met: bool,
    pub line_number: usize,
}

impl <'a> Liner<'a> {
    fn read_line(&mut self) -> Vec<Token> {
        let mut line = vec![];

        loop {
            let next = self.backend.grab();
            line.push(next.clone());

            match next {
                Token::Newline => break,
                Token::End => {
                    self.end_token_met = true;
                    break;
                },
                _ => {},
            }
        }

        println!("Initial Tokens:");
        for it in &line {
            println!("    {:?}", it);
        }
        println!("");

        line = transform(&line, &transform_duplicate_whitespaces);
        line = transform(&line, &transform_numbers);
        line = transform(&line, &transform_tight_strings);

        line = line.iter()
            .filter(|&it| match it {
                Token::Whitespace { value: _ } => false,
                _ => true,
            })
            .cloned()
            .collect();

        return line;
    }

    pub fn new(
        backend: &'a mut (dyn Stream<Token> + 'a),
    ) -> Liner<'a> {
        return Liner::<'a> {
            backend: backend,
            end_token_met: false,
            line_number: 0,
        };
    }
}

impl <'a> Stream<Vec<Token>> for Liner<'a> {
    fn has_next(&self) -> bool {
        return !self.end_token_met;
    }

    fn grab(&mut self) -> Vec<Token> {
        self.line_number += 1;
        return self.read_line();
    }

    fn get_offset(&self) -> usize {
        return self.line_number;
    }
}
