// use std::io::Read;

mod cherry;
use cherry::nodes::*;
use cherry::{LeveledVisitor};

use ferris_says::say;

use std::io::{stdout, BufWriter};

struct SampleVisitor;

impl LeveledVisitor for SampleVisitor {
    fn visit_leaf(&mut self, it: &mut Leaf, data: usize) {
        println!("{}Leaf: {:?}", " ".repeat(data), it.value);
    }

    fn visit_binary(&mut self, it: &mut Binary, data: usize) {
        println!("{}Binary {{", " ".repeat(data));
        it.lefter.accept_leveled_visitor(self, data + 2);
        it.righter.accept_leveled_visitor(self, data + 2);
        println!("{}}}", " ".repeat(data));
    }

    fn visit_unary(&mut self, it: &mut Unary, data: usize) {
        println!("{}Unary {{", " ".repeat(data));
        it.operator.accept_leveled_visitor(self, data + 2);
        it.target.accept_leveled_visitor(self, data + 2);
        println!("{}}}", " ".repeat(data));
    }
}

fn main() {
    println!("Starting: ");

    let mut ast = cherry::parse();

    println!("AST: {:?}", ast.visualize());

    ast.accept_leveled_visitor(&mut SampleVisitor, 0);

    let stdout = stdout();
    let message = String::from("Done!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
