use ferris_says::say;

use std::io::{stdout, BufWriter};

use parsing::stream::stdin_stream::{StdinStream};
use parsing::stream::accumulator_stream::{SimpleAccumulatorStream};

use cash::Node;
use cash::lexer::{Lexer};
use cash::liner::{Liner};

fn main() {
    println!("Starting: ");

    let mut user_input = StdinStream::new();
    let mut accumulator_stream = SimpleAccumulatorStream::new(&mut user_input);
    let mut tokenizer = Lexer::new(&mut accumulator_stream);
    let mut liner = Liner::new(&mut tokenizer);

    loop {
        let mut ast = cash::parse(&mut liner);

        ast.accept_leveled_visitor(&mut cash::ASTPrinter, 0);

        let stdout = stdout();
        let message = String::from("Done!");
        let width = message.chars().count();

        let mut writer = BufWriter::new(stdout.lock());
        say(message.as_bytes(), width, &mut writer).unwrap();
    }
}
