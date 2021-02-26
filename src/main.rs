// use std::io::Read;

mod cherry;
use cherry::*;

use ferris_says::say;

use std::io::Read;
use std::io::{stdout, BufWriter};

use orders::stream::analyzable_stream::*;
use orders::stream::accumulator_stream::*;
use orders::stream::text_stream::*;
use orders::stream::buffered_stream::*;
use orders::stream::std_stream::*;
use orders::stream::Stream;

struct SampleVisitor;

impl LeveledVisitor for SampleVisitor {
    fn visit_leaf(&mut self, it: &mut Leaf, data: usize) {
        println!("{}Visiting: Leaf: {:?}", " ".repeat(data), it.value);
    }

    fn visit_binary(&mut self, it: &mut Binary, data: usize) {
        println!("{}Visiting: Binary {{", " ".repeat(data));
        it.lefter.accept_leveled_visitor(self, data + 2);
        it.righter.accept_leveled_visitor(self, data + 2);
        println!("{}Visiting: Binary }}", " ".repeat(data));
    }

    fn visit_unary(&mut self, it: &mut Unary, data: usize) {
        println!("{}Visiting: Unary {{", " ".repeat(data));
        it.operator.accept_leveled_visitor(self, data + 2);
        it.target.accept_leveled_visitor(self, data + 2);
        println!("{}Visiting: Unary }}", " ".repeat(data));
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
    println!("Starting: ");

    let input1 = StdinStream::new();
    let input2 = SimpleBufferedStream::new(Box::new(input1), 16, 5, Some('N'));
    let input3 = SimpleTextStream::new(input2);
    let mut input = SimpleAnalyzableStream::new(input3);

    while input.has_next() && input.peek() != Some('n') {
        input.step();
    }

    println!("Read: {}", input.revise_all());

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

    ast.accept_leveled_visitor(&mut visitor, 0);

    let stdout = stdout();
    let message = String::from("Done!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
