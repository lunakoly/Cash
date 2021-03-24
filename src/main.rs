use ferris_says::say;

use std::io::{stdout, BufWriter};

use parsing::stream::{Stream};
use parsing::stream::stdin_stream::{StdinStream};
use parsing::stream::accumulator_stream::{SimpleAccumulatorStream};

use cash::ast::*;
use cash::lexer::{Lexer};
use cash::parser::{Parser};

fn main() {
    println!("Starting: ");

    let mut user_input = StdinStream::new();
    let mut accumulator_stream = SimpleAccumulatorStream::new(&mut user_input);
    let mut tokenizer = Lexer::new(&mut accumulator_stream);
    let mut parser = Parser::new(&mut tokenizer);

    loop {
        if !parser.has_next() {
            break;
        }

        let wrapped = parser.grab();
        let mut ast = wrapped.borrow_mut();

        ast.accept_leveled_visitor(&mut cash::ast::ASTPrinter, 0);

        let stdout = stdout();
        let message = String::from("Done!");
        let width = message.chars().count();

        let mut writer = BufWriter::new(stdout.lock());
        say(message.as_bytes(), width, &mut writer).unwrap();
    }

    println!("BYE");
}
