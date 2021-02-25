// use std::io::Read;

mod cherry;
use cherry::*;

use ferris_says::say;

use std::io::Read;
use std::io::{stdout, BufWriter};

struct SampleVisitor;

impl SimpleVisitor for SampleVisitor {
    fn visit_leaf(&mut self, it: &mut Leaf) {
        println!("Visiting: Leaf: {:?}", it.value);
    }

    fn visit_binary(&mut self, it: &mut Binary) {
        println!("Visiting: Binary {{");
        it.lefter.accept_simple_visitor(self);
        it.righter.accept_simple_visitor(self);
        println!("Visiting: Binary }}");
    }

    fn visit_unary(&mut self, it: &mut Unary) {
        println!("Visiting: Unary {{");
        it.operator.accept_simple_visitor(self);
        it.target.accept_simple_visitor(self);
        println!("Visiting: Unary }}");
    }
}

macro_rules! within {
    ( $this:expr, $opening:expr, $closing:expr, $action:block ) => {
        if $this.accept($opening) {
            let it = $action;
            $this.accept($closing);
            return it;
        }
    }
}

#[allow(dead_code)]
struct Context {
    input: std::iter::Peekable<std::io::Bytes<std::io::Stdin>>,
    indent_level: u32,
    line_number: u32,
}

impl Context {
    fn skip_blank(&mut self) {
        while let Some(Ok(symbol)) = self.input.peek() {
            if
                *symbol as char == '\n' ||
                *symbol as char == '\t' ||
                *symbol as char == ' ' {
                self.input.next();
            } else {
                break;
            }
        }
    }

    fn expect(&mut self, next: char) -> bool {
        self.skip_blank();

        if let Some(Ok(symbol)) = self.input.peek() {
            *symbol as char == next
        } else {
            false
        }
    }

    fn accept(&mut self, next: char) -> bool {
        if self.expect(next) {
            self.input.next();
            true
        } else {
            false
        }
    }

    fn expect_in(&mut self, from: char, to: char) -> bool {
        self.skip_blank();

        if let Some(Ok(symbol)) = self.input.peek() {
            from as u8 <= *symbol && *symbol <= to as u8
        } else {
            false
        }
    }

    fn get(&mut self) -> char {
        if let Some(Ok(symbol)) = self.input.next() {
            symbol as char
        } else {
            0 as char
        }
    }

    #[allow(dead_code)]
    fn has_next(&mut self) -> bool {
        if let Some(Ok(_)) = self.input.peek() {
            true
        } else {
            false
        }
    }

    fn parse_leaf(&mut self) -> Box<dyn Node> {
        let mut lexeme = String::new();

        within! {self, '(', ')', {
            self.parse_plus_minus()
        }};

        while
            self.expect_in('0', '9') ||
            self.expect_in('a', 'z') ||
            self.expect_in('A', 'Z') ||
            self.expect('_') {
            lexeme.push(self.get());
        }

        within! {self, '(', ')', {
            Box::new(Unary {
                operator: Box::new(Leaf { value: lexeme }),
                target: self.parse_plus_minus(),
            })
        }};

        Box::new(Leaf { value: lexeme })
    }

    fn parse_plus_minus(&mut self) -> Box<dyn Node> {
        let mut it = self.parse_leaf();

        while
            self.expect('+') ||
            self.expect('-') {
            let that = Binary {
                lefter: it,
                operator: Box::new(Leaf { value: self.get().to_string() }),
                righter: self.parse_leaf(),
            };
            it = Box::new(that);
        }

        it
    }
}

fn main() {
    let mut context = Context {
        input: std::io::stdin().bytes().peekable(),
        indent_level: 0,
        line_number: 1,
    };

    let test = Leaf { value: String::from("Test") };
    let fest = Leaf { value: String::from("Fest") };

    let mest = Box::new(Binary {
        lefter: Box::new(test),
        operator: Box::new(Leaf { value: String::from("+") }),
        righter: Box::new(fest),
    });

    let nest = Box::new(Binary {
        lefter: Box::new(Leaf { value: String::from("nest") }),
        operator: Box::new(Leaf { value: "-".to_owned() }),
        righter: mest,
    });

    println!("Got: {:?}", nest.visualize());

    let mut ast = context.parse_plus_minus();

    println!("AST: {:?}", ast.visualize());

    let mut visitor = SampleVisitor;

    ast.accept_simple_visitor(&mut visitor);

    let stdout = stdout();
    let message = String::from("Done!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
