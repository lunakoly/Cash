include!(concat!(env!("OUT_DIR"), "/ast.rs"));

use orders::stream::analyzable_stream::*;
use orders::stream::accumulator_stream::*;
use orders::stream::Stream;

use orders::{within, within_parentheses};

use nodes::*;

#[allow(dead_code)]
struct Context {
    input: SimpleAnalyzableStream,
    indent_level: u32,
    line_number: u32,
}

impl Context {
    fn skip_blank(&mut self) {
        while let Some(symbol) = self.input.peek() {
            if
                symbol == '\n' ||
                symbol == '\t' ||
                symbol == ' ' {
                self.input.step();
            } else {
                break;
            }
        }
    }

    fn expect_in(&mut self, from: char, to: char) -> bool {
        if let Some(symbol) = self.input.peek() {
            // from as u32 <= symbol as u32 && symbol as u32 <= to as u32
            (from..to).contains(&symbol)
        } else {
            false
        }
    }

    fn expect_operator(&mut self, operator: char) -> bool {
        self.skip_blank();
        return Some(operator) == self.input.peek();
    }

    fn parse_identifier(&mut self) -> Box<dyn Node> {
        self.skip_blank();
        self.input.clear();

        while
            self.expect_in('0', '9') ||
            self.expect_in('a', 'z') ||
            self.expect_in('A', 'Z') ||
            self.input.peek() == Some('_') {
            self.input.step();
        }

        return Box::new(Leaf {
            value: self.input.revise_all()
        });
    }

    fn parse_leaf(&mut self) -> Box<dyn Node> {
        within_parentheses! { self =>
            self.parse_plus_minus()
        };

        let it = self.parse_identifier();

        within_parentheses! { self =>
            Box::new(Unary {
                operator: it,
                target: self.parse_plus_minus(),
            })
        };

        return it;
    }

    fn parse_plus_minus(&mut self) -> Box<dyn Node> {
        let mut it = self.parse_leaf();

        while
            self.expect_operator('+') ||
            self.expect_operator('-') {
            let that = Binary {
                lefter: it,
                operator: Box::new(Leaf { value: self.input.grab().unwrap().to_string() }),
                righter: self.parse_leaf(),
            };
            it = Box::new(that);
        }

        return it;
    }
}

pub fn parse() -> Box<dyn Node> {
    let mut context = Context {
        input: SimpleAnalyzableStream::acquire(16, 5),
        indent_level: 0,
        line_number: 1,
    };

    return context.parse_plus_minus();
}
