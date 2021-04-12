pub mod ast;
pub mod lexer;
pub mod liner;
pub mod parser;
pub mod grammar;

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
            Token::Text {
                value: "echo".to_owned()
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Text {
                value: "Hello".to_owned()
            },
            Token::Operator {
                value: ",".to_owned()
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Text {
                value: "World".to_owned()
            },
            Token::Operator {
                value: "!".to_owned()
            },
            Token::End,
        ]);
    }

    #[test]
    fn test_lexer_concatenation() {
        assert_tokens("a,b a, b ,a", &[
            Token::Text {
                value: "a,b".to_owned()
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Text {
                value: "a".to_owned()
            },
            Token::Operator {
                value: ",".to_owned()
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Text {
                value: "b".to_owned()
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Text {
                value: ",a".to_owned()
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
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Number {
                value: "10".to_owned(),
                base: 2,
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Number {
                value: "10".to_owned(),
                base: 8,
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Number {
                value: "10".to_owned(),
                base: 16,
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Number {
                value: "3".to_owned(),
                base: 10,
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Text {
                value: "3b".to_owned()
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Number {
                value: "9".to_owned(),
                base: 10,
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Text {
                value: "9b".to_owned()
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Text {
                value: "9o".to_owned()
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Text {
                value: "f".to_owned()
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Text {
                value: "fb".to_owned()
            },
            Token::Whitespace {
                value: " ".to_owned()
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
            Token::Text {
                value: "a".to_owned()
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Operator {
                value: "+".to_owned()
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Text {
                value: "b".to_owned()
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Text {
                value: "a+b".to_owned()
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Text {
                value: "a".to_owned()
            },
            Token::Operator {
                value: "+".to_owned()
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Text {
                value: "+b".to_owned()
            },
            Token::Whitespace {
                value: " ".to_owned()
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
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Number {
                value: "2".to_owned(),
                base: 10
            },
            Token::Operator {
                value: "+".to_owned()
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Operator {
                value: "+".to_owned()
            },
            Token::Number {
                value: "1".to_owned(),
                base: 10
            },
            Token::Whitespace {
                value: " ".to_owned()
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
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Text {
                value: "a".to_owned()
            },
            Token::Delimiter {
                value: "(".to_owned()
            },
            Token::Text {
                value: "b".to_owned()
            },
            Token::Delimiter {
                value: ")".to_owned()
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Text {
                value: "a-b".to_owned()
            },
            Token::Operator {
                value: "-".to_owned()
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Text {
                value: "a".to_owned()
            },
            Token::Operator {
                value: "+".to_owned()
            },
            Token::Number {
                value: "7".to_owned(),
                base: 10
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Text {
                value: "7+b".to_owned()
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
            Token::Text {
                value: "variable".to_owned()
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Delimiter {
                value: "@".to_owned()
            },
            Token::Text {
                value: "descriptor".to_owned()
            },
            Token::End,
        ]);
    }

    #[test]
    fn test_lexer_tight() {
        assert_tokens("echo test++++fest", &[
            Token::Text {
                value: "echo".to_owned()
            },
            Token::Whitespace {
                value: " ".to_owned()
            },
            Token::Text {
                value: "test++++fest".to_owned()
            },
            Token::End,
        ]);
    }
}
