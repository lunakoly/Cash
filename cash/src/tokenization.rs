use parsing::stream::analyzable_stream::{AnalyzableStream};

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Operator {
        value: String
    },
    NumberSegment {
        value: String,
        base: u8
    },
    Number {
        value: String,
        base: u8
    },
    String {
        value: String
    },
    Whitespace {
        value: String
    },
    Newline,
}

pub const OPERATORS: &'static str = ":=+-*%$#@!&^|/.~()[]{}<>;,";

struct Context<'a> {
    tokens: Vec<Token>,
    stream: &'a mut (dyn AnalyzableStream + 'a),
}

fn is_whitespace(symbol: char) -> bool {
    return " \t".contains(symbol);
}

fn is_opearator(symbol: char) -> bool {
    return OPERATORS.contains(symbol);
}

fn is_binary(symbol: char) -> bool {
    return ('0'..='1').contains(&symbol);
}

fn is_octal(symbol: char) -> bool {
    return ('0'..='7').contains(&symbol);
}

fn is_decimal(symbol: char) -> bool {
    return ('0'..='9').contains(&symbol);
}

fn is_hexadecimal(symbol: char) -> bool {
    return
        ('0'..='9').contains(&symbol) ||
        ('a'..='f').contains(&symbol) ||
        ('A'..='F').contains(&symbol);
}

fn is_implicit_string_content(symbol: char) -> bool {
    return
        !is_opearator(symbol) &&
        !is_whitespace(symbol) &&
        symbol != '\n';
}

fn read_escape(context: &mut Context) {
    if context.stream.peek() == Some('\n') {
        context.tokens.push(Token::Newline);
    }
}

fn read_whitespace(context: &mut Context) {
    while let Some(symbol) = context.stream.peek() {
        if is_whitespace(symbol) {
            context.stream.step();
        } else {
            break;
        }
    }

    context.tokens.push(
        Token::Whitespace { value: context.stream.revise_all() }
    );
}

fn read_binary(context: &mut Context) {
    while let Some(symbol) = context.stream.peek() {
        if is_binary(symbol) {
            context.stream.step();
        } else {
            break;
        }
    }

    context.tokens.push(
        Token::NumberSegment {
            value: context.stream.revise_all(),
            base: 2,
        }
    );
}

fn read_octal(context: &mut Context) {
    while let Some(symbol) = context.stream.peek() {
        if is_octal(symbol) {
            context.stream.step();
        } else {
            break;
        }
    }

    context.tokens.push(
        Token::NumberSegment {
            value: context.stream.revise_all(),
            base: 8,
        }
    );
}

fn read_decimal(context: &mut Context) {
    while let Some(symbol) = context.stream.peek() {
        if is_decimal(symbol) {
            context.stream.step();
        } else {
            break;
        }
    }

    context.tokens.push(
        Token::NumberSegment {
            value: context.stream.revise_all(),
            base: 10,
        }
    );
}

fn read_hexadecimal(context: &mut Context) {
    while let Some(symbol) = context.stream.peek() {
        if is_hexadecimal(symbol) {
            context.stream.step();
        } else {
            break;
        }
    }

    context.tokens.push(
        Token::NumberSegment {
            value: context.stream.revise_all(),
            base: 16,
        }
    );
}

fn read_implicit_string(context: &mut Context) {
    while let Some(symbol) = context.stream.peek() {
        if is_implicit_string_content(symbol) {
            context.stream.step();
        } else {
            break;
        }
    }

    context.tokens.push(
        Token::String { value: context.stream.revise_all() }
    );
}

pub fn tokenize<'a>(
    stream: &'a mut (dyn AnalyzableStream + 'a)
) -> Vec<Token> {
    if !stream.has_next() {
        return vec![];
    }

    let mut context = Context {
        tokens: vec![],
        stream: stream,
    };

    while let Some(symbol) = context.stream.peek() {
        context.stream.clear();

        if symbol == '\\' {
            context.stream.step();
            read_escape(&mut context);
        } else if symbol == '\n' {
            context.tokens.push(Token::Newline);
            context.stream.step();
            break;
        } else if is_whitespace(symbol) {
            context.stream.step();
            read_whitespace(&mut context);
        } else if is_opearator(symbol) {
            context.stream.step();
            context.tokens.push(Token::Operator { value: String::from(symbol) });
        } else if is_binary(symbol) {
            context.stream.step();
            read_binary(&mut context);
        } else if is_octal(symbol) {
            context.stream.step();
            read_octal(&mut context);
        } else if is_decimal(symbol) {
            context.stream.step();
            read_decimal(&mut context);
        } else if is_hexadecimal(symbol) {
            context.stream.step();
            read_hexadecimal(&mut context);
        } else {
            context.stream.step();
            read_implicit_string(&mut context);
        }
    }

    println!("Initial Tokens:");
    for it in &context.tokens {
        println!("    {:?}", it);
    }
    println!("");

    return context.tokens;
}

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

    println!("Got Tokens:");
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
            if *base < 10 {
                tokens.pop();
                tokens.push(
                    Token::Number {
                        value: last_value.clone(),
                        base: 10,
                    }
                );
            } else if *base > 10 {
                tokens.pop();
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

pub fn process_input<'a>(
    stream: &'a mut (dyn AnalyzableStream + 'a)
) -> Vec<Token> {
    let mut tokens = tokenize(stream);

    tokens = transform(&tokens, &transform_duplicate_whitespaces);
    tokens = transform(&tokens, &transform_numbers);
    tokens = transform(&tokens, &transform_tight_strings);

    tokens = tokens.iter()
        .filter(|&it| match it {
            Token::Whitespace { value: _ } => false,
            _ => true,
        })
        .cloned()
        .collect();

    return tokens;
}
