use ferris_says::say;

use std::io::{stdout, BufWriter};
use std::io::{stdin};

use parsing::stream::{Stream};
use parsing::stream::stdin_stream::{StdinStream};
use parsing::stream::accumulator_stream::{SimpleAccumulatorStream};

use cash::ast::*;
use cash::lexer::{Lexer};
use cash::parser::{Parser};

use processing::{launch_pipeline, launch_input_substitution, launch_output_substitution};

use terminals::terminal_stream::TerminalStream;

fn test_processing() -> std::io::Result<()> {
    println!("Testing Processing:");

    // let result = launch_command(&["E:\\Projects\\Other\\rust_sandbox\\processing\\samples\\test.bat"])?
    //     .wait_with_output()?;

    let result = launch_pipeline(
        None,
        Some(std::process::Stdio::piped()),
        &[
            &["E:\\Projects\\Other\\rust_sandbox\\processing\\samples\\test.bat"],
        ]
    )?
        .wait_with_output()?;

    let output = String::from_utf8_lossy(&result.stdout);
    println!("Got: {:?}", &output);
    Ok(())
}

fn main() {
    test_processing().unwrap();

    println!("Starting: ");

    // let mut user_input = StdinStream::new();
    let mut user_input = TerminalStream::new();

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
