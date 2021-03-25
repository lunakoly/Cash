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
    /// If true, peek() will read one
    /// more token. Otherwise, peek() will
    /// return `last_token`.
    pub should_read: bool,
}

impl <'a> Lexer<'a> {
    fn read_escape(&mut self) {
        if self.backend.accept('\n') {
            self.last_token = Token::Newline;
        } else {
            self.read_item();
        }
    }

    fn read_whitespace(&mut self) {
        while let Some(symbol) = self.backend.peek() {
            if is_whitespace(symbol) {
                self.backend.step();
            } else {
                break;
            }
        }

        self.last_token = Token::Whitespace {
            value: self.backend.revise_all()
        };
    }

    fn read_binary(&mut self) {
        while let Some(symbol) = self.backend.peek() {
            if is_binary(symbol) {
                self.backend.step();
            } else {
                break;
            }
        }

        self.last_token = Token::NumberSegment {
            value: self.backend.revise_all(),
            base: 2,
        };
    }

    fn read_octal(&mut self) {
        while let Some(symbol) = self.backend.peek() {
            if is_octal(symbol) {
                self.backend.step();
            } else {
                break;
            }
        }

        self.last_token = Token::NumberSegment {
            value: self.backend.revise_all(),
            base: 8,
        };
    }

    fn read_decimal(&mut self) {
        while let Some(symbol) = self.backend.peek() {
            if is_decimal(symbol) {
                self.backend.step();
            } else {
                break;
            }
        }

        self.last_token = Token::NumberSegment {
            value: self.backend.revise_all(),
            base: 10,
        };
    }

    fn read_hexadecimal(&mut self) {
        while let Some(symbol) = self.backend.peek() {
            if is_decimal(symbol) {
                self.backend.step();
            } else {
                break;
            }
        }

        self.last_token = Token::NumberSegment {
            value: self.backend.revise_all(),
            base: 16,
        };
    }

    fn read_implicit_string(&mut self) {
        while let Some(symbol) = self.backend.peek() {
            if is_implicit_string_content(symbol) {
                self.backend.step();
            } else {
                break;
            }
        }

        self.last_token = Token::String {
            value: self.backend.revise_all()
        };
    }

    fn read_item(&mut self) {
        if self.backend.accept('\\') {
            self.read_escape();
        } else if self.backend.accept('\n') {
            self.last_token = Token::Newline;
        } else if let Some(symbol) = self.backend.peek() {
            if is_whitespace(symbol) {
                self.backend.step();
                self.read_whitespace();
            } else if is_operator(symbol) {
                self.backend.step();
                self.last_token = Token::Operator {
                    value: String::from(symbol)
                };
            } else if is_delimiter(symbol) {
                self.backend.step();
                self.last_token = Token::Delimiter {
                    value: String::from(symbol)
                };
            } else if is_binary(symbol) {
                self.backend.step();
                self.read_binary();
            } else if is_octal(symbol) {
                self.backend.step();
                self.read_octal();
            } else if is_decimal(symbol) {
                self.backend.step();
                self.read_decimal();
            } else if is_hexadecimal(symbol) {
                self.backend.step();
                self.read_hexadecimal();
            } else {
                self.backend.step();
                self.read_implicit_string();
            }
        } else {
            self.last_token = Token::End;
        }
    }

    fn read_token(&mut self) {
        self.backend.clear();
        self.last_token_offset = self.backend.get_offset();
        self.read_item();
        self.should_read = false;
    }

    pub fn new(
        backend: &'a mut (dyn AccumulatorStream + 'a),
    ) -> Lexer<'a> {
        return Lexer::<'a> {
            backend: backend,
            last_token: Token::End,
            last_token_offset: 0,
            should_read: true
        };
    }
}

impl <'a> Stream<Token> for Lexer<'a> {
    fn get_end_value(&self) -> Token {
        return Token::End;
    }

    fn peek(&mut self) -> Token {
        if self.should_read {
            self.read_token();
        }

        return self.last_token.clone();
    }

    fn step(&mut self) {
        if self.should_read {
            self.read_token();
        }

        self.should_read = true;
    }

    fn get_offset(&self) -> usize {
        return self.last_token_offset;
    }
}
