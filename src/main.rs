use ferris_says::say;

use std::io::{stdout, BufWriter};

use parsing::stream::stdin_stream::{StdinStream};
use parsing::stream::burst_stream::{SimpleBurstStream};
use parsing::stream::analyzable_stream::{SimpleAnalyzableStream};

use cash::Node;

fn main() {
    println!("Starting: ");

    let mut raw_user_input = StdinStream::new();
    let mut user_input = SimpleBurstStream::new(&mut raw_user_input, 16);
    let mut analyzable_stream = SimpleAnalyzableStream::acquire(16, 5, &mut user_input);

    loop {
        let mut ast = cash::parse(&mut analyzable_stream);

        ast.accept_leveled_visitor(&mut cash::ASTPrinter, 0);

        let stdout = stdout();
        let message = String::from("Done!");
        let width = message.chars().count();

        let mut writer = BufWriter::new(stdout.lock());
        say(message.as_bytes(), width, &mut writer).unwrap();
    }
}
