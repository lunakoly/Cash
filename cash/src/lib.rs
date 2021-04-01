pub mod ast;
pub mod lexer;
pub mod liner;
pub mod value;
pub mod parser;
pub mod grammar;
pub mod runner;

#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, Token};
    use crate::liner::{Liner};

    use parsing::stream::*;
    use parsing::stream::wrapper_stream::{WrapperStream};
    use parsing::stream::accumulator_stream::{SimpleAccumulatorStream};

    fn assert_tokens(input: &str, expected: &[Token]) {
        let mut input_stream = WrapperStream::new(
            input.as_bytes()
        );
        let mut accumulator_stream = SimpleAccumulatorStream::new(&mut input_stream);
        let mut tokenizer = Lexer::new(&mut accumulator_stream);
        let mut liner = Liner::new(&mut tokenizer);

        assert!(liner.has_next());

        let got = liner.grab();

        assert_eq!(expected[..], got[..]);
    }

    #[test]
    fn test_lexer_trivial() {
        assert_tokens("echo Hello, World!", &[
            Token::String {
                value: "echo".to_owned()
            },
            Token::String {
                value: "Hello,".to_owned()
            },
            Token::String {
                value: "World!".to_owned()
            },
            Token::End,
        ]);
    }

    #[test]
    fn test_lexer_numeric() {
        assert_tokens("10 10b 10o 10h 3 3b 9 9b 9o f fb fh", &[
            Token::Number {
                value: "10".to_owned(),
                base: 10,
            },
            Token::Number {
                value: "10".to_owned(),
                base: 2,
            },
            Token::Number {
                value: "10".to_owned(),
                base: 8,
            },
            Token::Number {
                value: "10".to_owned(),
                base: 16,
            },
            Token::Number {
                value: "3".to_owned(),
                base: 10,
            },
            Token::String {
                value: "3b".to_owned()
            },
            Token::Number {
                value: "9".to_owned(),
                base: 10,
            },
            Token::String {
                value: "9b".to_owned()
            },
            Token::String {
                value: "9o".to_owned()
            },
            Token::String {
                value: "f".to_owned()
            },
            Token::String {
                value: "fb".to_owned()
            },
            Token::Number {
                value: "f".to_owned(),
                base: 16,
            },
            Token::End,
        ]);
    }

    #[test]
    fn test_lexer_arithmetics() {
        assert_tokens("a + b a+b a+ +b 1+2 2+ +1 +++ a(b) a-b- a+7 7+b", &[
            Token::String {
                value: "a".to_owned()
            },
            Token::Operator {
                value: "+".to_owned()
            },
            Token::String {
                value: "b".to_owned()
            },
            Token::String {
                value: "a+b".to_owned()
            },
            Token::String {
                value: "a+".to_owned()
            },
            Token::String {
                value: "+b".to_owned()
            },
            Token::Number {
                value: "1".to_owned(),
                base: 10
            },
            Token::Operator {
                value: "+".to_owned()
            },
            Token::Number {
                value: "2".to_owned(),
                base: 10
            },
            Token::Number {
                value: "2".to_owned(),
                base: 10
            },
            Token::Operator {
                value: "+".to_owned()
            },
            Token::Operator {
                value: "+".to_owned()
            },
            Token::Number {
                value: "1".to_owned(),
                base: 10
            },
            Token::Operator {
                value: "+".to_owned()
            },
            Token::Operator {
                value: "+".to_owned()
            },
            Token::Operator {
                value: "+".to_owned()
            },
            Token::String {
                value: "a".to_owned()
            },
            Token::Delimiter {
                value: "(".to_owned()
            },
            Token::String {
                value: "b".to_owned()
            },
            Token::Delimiter {
                value: ")".to_owned()
            },
            Token::String {
                value: "a-b-".to_owned()
            },
            Token::String {
                value: "a+7".to_owned()
            },
            Token::Number {
                value: "7".to_owned(),
                base: 10
            },
            Token::String {
                value: "+b".to_owned()
            },
            Token::End,
        ]);
    }

    #[test]
    fn test_lexer_controls() {
        assert_tokens("$variable @descriptor", &[
            Token::Delimiter {
                value: "$".to_owned()
            },
            Token::String {
                value: "variable".to_owned()
            },
            Token::Delimiter {
                value: "@".to_owned()
            },
            Token::String {
                value: "descriptor".to_owned()
            },
            Token::End,
        ]);
    }
}
