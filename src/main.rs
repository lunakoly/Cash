// use std::io::Read;

mod cherry;
// use cherry::nodes::*;
use cherry::{ASTPrinter};

use ferris_says::say;

use std::io::{stdout, BufWriter};

fn main() {
    println!("Starting: ");

    let mut ast = cherry::parse();
    ast.accept_leveled_visitor(&mut ASTPrinter, 0);

    let stdout = stdout();
    let message = String::from("Done!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
