// use std::io::Read;

mod cherry;
// use cherry::nodes::*;
use cherry::{ASTPrinter};

use ferris_says::say;

use std::io::{stdout, BufWriter};

use orders::stream::stdin_stream::{StdinStream};
use orders::stream::analyzable_stream::{SimpleAnalyzableStream};

fn main() {
    println!("Starting: ");

    let mut raw_user_input = StdinStream::new();
    let mut analyzable_stream = SimpleAnalyzableStream::acquire(16, 5, &mut raw_user_input);

    let mut ast = cherry::parse(&mut analyzable_stream);

    ast.accept_leveled_visitor(&mut ASTPrinter, 0);

    let stdout = stdout();
    let message = String::from("Done!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
