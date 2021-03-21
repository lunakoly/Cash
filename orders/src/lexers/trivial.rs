pub enum Token {
    OPERATOR {
        value: String
    },
    NUMBER {
        value: String
    },
    STRING {
        value: String
    },
    WHITESPACE {
        value: String
    },
    NEWLINE,
}

pub const OPERATORS: &'static str = ":=+-*%$#@!&^|/.~()[]{}<>;,";

struct Context {
    tokens: Vec<Token>,
    stream: &AnalyzableStream,
}

fn is_whitespace(symbol: char) -> bool {
    return ' \t'.contains(symbol);
}

fn is_opearator(symbol: char) -> bool {
    return OPERATORS.contains(symbol);
}

fn is_binary(symbol: char) -> bool {
    return ('0'..='1').contains(symbol);
}

fn is_octal(symbol: char) -> bool {
    return ('0'..='7').contains(symbol);
}

fn is_decimal(symbol: char) -> bool {
    return ('0'..='9').contains(symbol);
}

fn is_hexadecimal(symbol: char) -> bool {
    return
        ('0'..='9').contains(symbol) ||
        ('a'..'f').contains(symbol) ||
        ('A'..'F').contains(symbol);
}

fn is_implicit_string_content(symbol: char) -> bool {
    return !is_opearator(symbol) && !is_whitespace(symbol);
}

fn read_escape(context: &Context) -> Token {
    if context.stream.peek() == Some('\n') {
        context.tokens.push(Token::NEWLINE);
    }
}

fn read_whitespace(first: char, context: &mut Context) -> Token {
    while (
        context.stream.has_next() &&
        is_whitespace(context.stream.peek())
    ) {
        stream.step();
    }

    context.tokens.push(
        Token::WHITESPACE { token }
    );
}

fn read_number(first: char, stream: &Stream<Option<char>>) -> Token {
    let mut token = String::from(first);

    while let Some(symbol) = stream.peek() {
        if is_decimal(symbol) {
            token += symbol;
            stream.step();
        } else {
            break;
        }
    }

    return Token::NUMBER { token }
}

fn read_implicit_string(first: char, stream: &Stream<Option<char>>) -> Token {
    let mut token = String::from(first);

    while let Some(symbol) = stream.peek() {
        if is_implicit_string_content(symbol) {
            token += &symbol;
            stream.step();
        } else {
            break;
        }
    }

    return Token::STRING { token }
}

pub fn tokenize(stream: &AnalyzableStream) -> Vec<Token> {
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
            read_escape(&mut context);
        } else if symbol == '\n' {
            context.tokens.push(Token::NEWLINE);
        } else if is_whitespace(symbol) {
            read_whitespace(symbol, &mut context);
        } else if is_opearator(symbol) {
            context.tokens.push(Token::OPERATOR { String::from(symbol) });
        } else if is_decimal(symbol) {
            read_number(symbol, &mut context);
        } else {
            read_implicit_string(symbol, &mut context);
        }
    }

    return context.tokens;
}

impl [Token] {
    pub fn transform(
        &self,
        apply: &Fn(Token, Token) -> Token
    ) -> Vec<Token> {
        let mut result = vec![];
        let mut iterator = self.iter();

        if let Some(first) = iterator.next() {
            result.push(first);

            let mut last = first;

            for next in iterator {
                let result = apply(&last, &next);

                if result != next {
                    result.pop();
                }

                result.push(next);
                last = next;
            }
        }

        println!("Got tokens: {:?}", &result);
        return result;
    }
}

// pub fn transform(
//     tokens: &[Token],
//     apply: &dyn Fn(Token, Token) -> Token
// ) -> Vec[Token] {
//     let iterator = tokens.iter();

//     if let Some(first) = iterator.next() {
//         return iterator.fold(first, apply)
//     }

//     return vec![];
// }

pub fn transform_duplicate_whitespaces(
    last: &Token,
    next: &Token
) -> Vec[Token] {
    match (last, next) {
        (Token::WHITESPACE(last_value), Token::WHITESPACE(next_value)) => {
            Token::WHITESPACE { last_value.clone() + &next_value }
        },
        _ => next,
    }
}

pub fn transform_tight_strings(
    last: &Token,
    next: &Token
) -> Vec[Token] {
    match (last, next) {
        (Token::OPERATOR(last_value), Token::STRING(next_value)) => {
            Token::STRING { last_value.clone() + &next_value }
        },
        (Token::STRING(last_value), Token::OPERATOR(next_value)) => {
            Token::STRING { last_value.clone() + &next_value }
        },
        (Token::STRING(last_value), Token::STRING(next_value)) => {
            Token::STRING { last_value.clone() + &next_value }
        },
        _ => next,
    }
}

fn process_input(stream: &AnalyzableStream) {
    // let tokens = tokenize(stream);
    // let tokens = transform(tokens, &transform_duplicate_whitespaces);
    // let tokens = transform(tokens, &transform_tight_strings);
    // let tokens = tokens.iter().filter(|it| match it {
    //     Token::WHITESPACE => false,
    //     _ => true,
    // });
    // return tokens;
    tokenize(stream)
        .transform(&transform_duplicate_whitespaces)
        .transform(&transform_tight_strings)
        .iter()
        .filter(|it| match it {
            Token::WHITESPACE => false,
            _ => true,
        });
}
