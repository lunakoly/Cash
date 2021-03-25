use parsing::stream::*;
use parsing::stream::accumulator_stream::*;

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum Token {
    Operator {
        value: String
    },
    Delimiter {
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
    End,
}

/// Operators are symbols that get clued
/// to the strings if there's no whitespace
/// between them
pub const OPERATORS: &'static str = ":=+-*%#!&^|/.~[]{}<>;,";
/// Delimiters never clue to strings
pub const DELIMITERS: &'static str = "()$@";

fn is_whitespace(symbol: char) -> bool {
    return " \t".contains(symbol);
}

fn is_operator(symbol: char) -> bool {
    return OPERATORS.contains(symbol);
}

fn is_delimiter(symbol: char) -> bool {
    return DELIMITERS.contains(symbol);
}

fn is_control(symbol: char) -> bool {
    return is_operator(symbol) || is_delimiter(symbol);
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
        !is_control(symbol) &&
        !is_whitespace(symbol) &&
        symbol != '\n';
}

/// Splits the input into tokens.
pub struct Lexer<'a> {
    /// Delegate for all operations.
    pub backend: &'a mut (dyn AccumulatorStream + 'a),
    /// Last parsed token.
    pub last_token: Token,
    /// Number of the character
    /// the last token starts with.
    pub last_token_offset: usize,
}

impl <'a> Lexer<'a> {
    fn read_escape(&mut self) -> Token {
        if self.backend.accept('\n') {
            Token::Newline
        } else {
            self.read_item()
        }
    }

    fn read_whitespace(&mut self) -> Token {
        while let Some(symbol) = self.backend.peek() {
            if is_whitespace(symbol) {
                self.backend.step();
            } else {
                break;
            }
        }

        return Token::Whitespace {
            value: self.backend.revise_all()
        };
    }

    fn read_binary(&mut self) -> Token {
        while let Some(symbol) = self.backend.peek() {
            if is_binary(symbol) {
                self.backend.step();
            } else {
                break;
            }
        }

        return Token::NumberSegment {
            value: self.backend.revise_all(),
            base: 2,
        };
    }

    fn read_octal(&mut self) -> Token {
        while let Some(symbol) = self.backend.peek() {
            if is_octal(symbol) {
                self.backend.step();
            } else {
                break;
            }
        }

        return Token::NumberSegment {
            value: self.backend.revise_all(),
            base: 8,
        };
    }

    fn read_decimal(&mut self) -> Token {
        while let Some(symbol) = self.backend.peek() {
            if is_decimal(symbol) {
                self.backend.step();
            } else {
                break;
            }
        }

        return Token::NumberSegment {
            value: self.backend.revise_all(),
            base: 10,
        };
    }

    fn read_hexadecimal(&mut self) -> Token {
        while let Some(symbol) = self.backend.peek() {
            if is_decimal(symbol) {
                self.backend.step();
            } else {
                break;
            }
        }

        return Token::NumberSegment {
            value: self.backend.revise_all(),
            base: 16,
        };
    }

    fn read_implicit_string(&mut self) -> Token {
        while let Some(symbol) = self.backend.peek() {
            if is_implicit_string_content(symbol) {
                self.backend.step();
            } else {
                break;
            }
        }

        return Token::String {
            value: self.backend.revise_all()
        };
    }

    fn read_item(&mut self) -> Token {
        self.backend.clear();

        if self.backend.accept('\\') {
            return self.read_escape();
        }

        if self.backend.accept('\n') {
            return Token::Newline;
        }

        if let Some(symbol) = self.backend.peek() {
            if is_whitespace(symbol) {
                self.backend.step();
                return self.read_whitespace();
            }

            if is_operator(symbol) {
                self.backend.step();
                return Token::Operator {
                    value: String::from(symbol)
                };
            }

            if is_delimiter(symbol) {
                self.backend.step();
                return Token::Delimiter {
                    value: String::from(symbol)
                };
            }

            if is_binary(symbol) {
                self.backend.step();
                return self.read_binary();
            }

            if is_octal(symbol) {
                self.backend.step();
                return self.read_octal();
            }

            if is_decimal(symbol) {
                self.backend.step();
                return self.read_decimal();
            }

            if is_hexadecimal(symbol) {
                self.backend.step();
                return self.read_hexadecimal();
            }

            self.backend.step();
            return self.read_implicit_string();
        }

        return Token::End;
    }

    pub fn new(
        backend: &'a mut (dyn AccumulatorStream + 'a),
    ) -> Lexer<'a> {
        return Lexer::<'a> {
            backend: backend,
            last_token: Token::End,
            last_token_offset: 0,
        };
    }
}

impl <'a> Stream<Token> for Lexer<'a> {
    fn has_next(&self) -> bool {
        return self.last_token != Token::End;
    }

    fn grab(&mut self) -> Token {
        self.last_token_offset = self.backend.get_offset();
        self.last_token = self.read_item();
        return self.last_token.clone();
    }

    fn get_offset(&self) -> usize {
        return self.last_token_offset;
    }
}
