// use ferris_says::say;

// use std::io::{stdout, BufWriter};
use std::io::Write;

use parsing::stream::{Stream};
use parsing::stream::stdin_stream::{StdinStream};
use parsing::stream::text_stream::{TextStream};
use parsing::stream::accumulator_stream::{SimpleAccumulatorStream};

// use frontend::ast::*;
use frontend::lexer::{Lexer};
use frontend::parser::{Parser};

// use processing::{launch_pipeline, launch_input_substitution, launch_output_substitution};

use terminals::is_interactive;
use terminals::terminal_stream::TerminalStream;

// use std::fs::File;

use backend::runner::Runner;

use backend::cast;
use backend::value;

use helpers::{elvis, result_or};

enum InputMode {
    Interactive {
        user_input: TerminalStream
    },
    OffScreen {
        user_input: StdinStream
    },
}

impl InputMode {
    fn get_stream(&mut self) -> &mut dyn TextStream {
        match self {
            InputMode::Interactive { user_input } => user_input,
            InputMode::OffScreen { user_input } => user_input,
        }
    }
}

fn main() {
    let mut user_input = if is_interactive() {
        println!("Welcome!");
        InputMode::Interactive {
            user_input: TerminalStream::new()
        }
    } else {
        InputMode::OffScreen {
            user_input: StdinStream::new()
        }
    };

    let mut accumulator_stream = SimpleAccumulatorStream::new(user_input.get_stream());
    let mut tokenizer = Lexer::new(&mut accumulator_stream);
    let mut parser = Parser::new(&mut tokenizer);
    let mut runner = Runner::new();

    loop {
        if !parser.has_next() {
            break;
        }

        if is_interactive() {
            print!("$ ");
            result_or! { std::io::stdout().flush() => break };
        }

        let wrapped = parser.grab();
        let mut ast = wrapped.borrow_mut();

        // if is_interactive() {
        //     ast.accept_leveled_visitor(&mut ASTPrinter, 0);
        // }

        ast.accept_simple_visitor(&mut runner);

        if is_interactive() {
            if let Some(string) = cast!(runner.value => value::string::StringValue) {
                println!("::: {:?} :::", string);
            } else if let Some(number) = cast!(runner.value => value::number::NumberValue) {
                println!("::: {:?} :::", number);
            } else if let Some(boolean) = cast!(runner.value => value::boolean::BooleanValue) {
                println!("::: {:?} :::", boolean);
            } else if let Some(closure) = cast!(runner.value => value::closure::ClosureValue) {
                println!("::: {:?} :::", closure);
            } else if let Some(scope) = cast!(runner.value => value::scope::ScopeValue) {
                println!("::: {:?} :::", scope);
            } else if let Some(none) = cast!(runner.value => value::none::NoneValue) {
                println!("::: {:?} :::", none);
            }
        }

        if runner.should_exit {
            break;
        }
    }
}
