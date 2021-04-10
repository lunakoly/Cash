use ferris_says::say;

use std::io::{stdout, BufWriter};

use parsing::stream::{Stream};
// use parsing::stream::stdin_stream::{StdinStream};
use parsing::stream::accumulator_stream::{SimpleAccumulatorStream};

use frontend::ast::*;
use frontend::lexer::{Lexer};
use frontend::parser::{Parser};

// use processing::{launch_pipeline, launch_input_substitution, launch_output_substitution};

use terminals::terminal_stream::TerminalStream;

// use std::fs::File;

use backend::runner::Runner;

use backend::cast;
use backend::value;

fn test_processing() -> std::io::Result<()> {
    // println!("Testing Processing:");

    // let runner.value = launch_pipeline(
    //     None,
    //     Some(std::process::Stdio::piped()),
    //     &[
    //         &["E:\\Projects\\Other\\rust_sandbox\\processing\\samples\\wrap.exe"],
    //         &["E:\\Projects\\Other\\rust_sandbox\\processing\\samples\\hide.exe"],
    //     ]
    // )?
    //     .wait_with_output()?;

    // let input = File::open("E:\\Projects\\Other\\rust_sandbox\\processing\\samples\\a.txt")?;
    // let output = File::create("E:\\Projects\\Other\\rust_sandbox\\processing\\samples\\c.txt")?;

    // let runner.value = launch_pipeline(
    //     Some(input),
    //     Some(output),
    //     &[
    //         &["E:\\Projects\\Other\\rust_sandbox\\processing\\samples\\wrap.exe"],
    //         &["E:\\Projects\\Other\\rust_sandbox\\processing\\samples\\hide.exe"],
    //     ]
    // )?
    //     .wait_with_output()?;

    // let output = String::from_utf8_lossy(&runner.value.stdout);
    // println!("Got: {:?}", &output);
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

    let mut runner = Runner::new();

    loop {
        if !parser.has_next() {
            break;
        }

        let wrapped = parser.grab();
        let mut ast = wrapped.borrow_mut();

        ast.accept_leveled_visitor(&mut frontend::ast::ASTPrinter, 0);

        ast.accept_simple_visitor(&mut runner);

        if let Some(string) = cast!(runner.value => value::string::StringValue) {
            println!("String ::: {:?}", string);
        } else if let Some(number) = cast!(runner.value => value::number::NumberValue) {
            println!("Number ::: {:?}", number);
        } else if let Some(boolean) = cast!(runner.value => value::boolean::BooleanValue) {
            println!("Boolean ::: {:?}", boolean);
        } else if let Some(none) = cast!(runner.value => value::none::NoneValue) {
            println!("::: None :::");
        }

        if runner.should_exit {
            break;
        }

        let stdout = stdout();
        let message = String::from("Done!");
        let width = message.chars().count();

        let mut writer = BufWriter::new(stdout.lock());
        say(message.as_bytes(), width, &mut writer).unwrap();
    }

    println!("BYE");
}
