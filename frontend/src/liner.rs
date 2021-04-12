use parsing::stream::*;

use crate::lexer::{Token};

use helpers::{elvis, some_or};

pub fn transform(
    tokens: &[Token],
    apply: &dyn Fn(&Token, &Token, &mut Vec<Token>) -> ()
) -> Vec<Token> {
    let mut result = vec![];
    let mut iterator = tokens.iter().rev();

    let first = some_or! { iterator.next() => return result };
    let mut last = (*first).clone();
    result.push(last.clone());

    for next in iterator {
        apply(&next, &last, &mut result);

        // in fact, this should always
        // succeed
        if let Some(value) = result.last() {
            last = value.clone();
        }
    }

    // println!("After Some Transformation:");
    // for it in &result {
    //     println!("    {:?}", it);
    // }
    // println!("");

    result.reverse();
    return result;
}

pub fn suffix_to_base(suffix: &str) -> Option<u8> {
    match suffix {
        "b" => Some(2),
        "o" => Some(8),
        "h" => Some(16),
        _ => None,
    }
}

fn transform_numbers(
    lefter: &Token,
    righter: &Token,
    target: &mut Vec<Token>
) {
    match (lefter, righter) {
        (
            Token::NumberSegment { value: lefter_value, base: lefter_base },
            Token::Number { value: righter_value, base: righter_base }
        ) => {
            // it's only possible that the 2 tokens
            // have different base

            // special case: when the second number
            // is 'b'_16 that must be treated as
            // 'b' suffix of a binary number

            target.pop();

            if righter_value == "b" && *lefter_base == 2 {
                target.push(lefter.clone());
                return;
            }

            if *lefter_base > *righter_base {
                target.push(
                    Token::Text {
                        value: lefter_value.clone() + &righter_value
                    }
                );
                return;
            }

            target.push(
                Token::Number {
                    value: lefter_value.clone() + &righter_value,
                    base: *righter_base
                }
            );
        },
        (
            Token::NumberSegment { value: lefter_value, base },
            Token::Text { value: righter_value }
        ) => {
            target.pop();

            if let Some(desired_base) = suffix_to_base(righter_value) {
                if *base <= desired_base {
                    target.push(
                        Token::Number {
                            value: lefter_value.clone(),
                            base: desired_base
                        }
                    );
                    return;
                }

                target.push(
                    Token::Text {
                        value: lefter_value.clone() + &righter_value
                    }
                );
                return;
            }

            target.push(
                Token::Text {
                    value: lefter_value.clone() + &righter_value
                }
            );
        },
        (
            Token::Text { value: lefter_value },
            Token::NumberSegment { value: righter_value, base: _ }
        ) => {
            target.pop();
            target.push(
                Token::Text {
                    value: lefter_value.clone() + &righter_value
                }
            );
        },
        (
            Token::NumberSegment { value: lefter_value, base },
            _
        ) => {
            if *base <= 10 {
                target.push(
                    Token::Number {
                        value: lefter_value.clone(),
                        base: 10,
                    }
                );
            } else {
                target.push(
                    Token::Text {
                        value: lefter_value.clone()
                    }
                );
            }
        },
        _ => {
            target.push(lefter.clone());
        },
    }
}

fn transform_tight_tokens(
    lefter: &Token,
    righter: &Token,
    target: &mut Vec<Token>
) {
    match (lefter, righter) {
        (
            Token::Text { value: lefter_value },
            Token::Text { value: righter_value }
        ) => {
            target.pop();
            target.push(
                Token::Text {
                    value: lefter_value.clone() + &righter_value
                }
            );
        },
        (
            Token::Whitespace { value: lefter_value },
            Token::Whitespace { value: righter_value }
        ) => {
            target.pop();
            target.push(
                Token::Whitespace {
                    value: lefter_value.clone() + &righter_value
                }
            );
        },
        (
            Token::Operator { value: lefter_value },
            Token::Text { value: righter_value }
        ) => {
            target.pop();
            target.push(
                Token::Text {
                    value: lefter_value.clone() + &righter_value
                }
            );
        },
        (
            Token::Number { value: lefter_value, base: _ },
            Token::Text { value: righter_value }
        ) => {
            target.pop();
            target.push(
                Token::Text {
                    value: lefter_value.clone() + &righter_value
                }
            );
        },
        (
            Token::Text { value: lefter_value },
            Token::Number { value: righter_value, base: _ }
        ) => {
            target.pop();
            target.push(
                Token::Text {
                    value: lefter_value.clone() + &righter_value
                }
            );
        },
        _ => {
            target.push(lefter.clone());
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
                Token::CommandEnd => break,
                Token::End => {
                    self.end_token_met = true;
                    break;
                },
                _ => {},
            }
        }

        // println!("Initial Tokens:");
        // for it in &line {
        //     println!("    {:?}", it);
        // }
        // println!("");

        line = transform(&line, &transform_numbers);
        line = transform(&line, &transform_tight_tokens);

        // line = line.iter()
        //     // .filter(|&it| match it {
        //     //     Token::Whitespace { value: _ } => false,
        //     //     _ => true,
        //     // })
        //     .cloned()
        //     .collect();

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
