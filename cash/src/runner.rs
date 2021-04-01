use crate::ast::*;
use crate::ast::nodes::*;

use crate::value::Value;
use crate::value::none::NoneValue;
use crate::value::number::NumberValue;
use crate::value::boolean::BooleanValue;
use crate::value::string::StringValue;

use processing::launch_pipeline;

use std::fs;

use crate::lexer::Token;

use crate::cast;

pub struct Runner {
    pub last_command: Vec<Box<dyn Value>>,
    pub should_exit: bool,
}

impl Runner {
    pub fn new() -> Runner {
        Runner {
            last_command: vec![],
            should_exit: false,
        }
    }
}

macro_rules! with_new_command {
    ( $this:ident => $( $it:stmt )* ) => {
        {
            let old_command = std::mem::replace(&mut $this.last_command, vec![]);
            $( $it )*
            std::mem::replace(&mut $this.last_command, old_command)
        }
    };
}

fn create_todo(location: &str) -> Box<dyn Value> {
    Box::new(
        StringValue::new(&("[todo:".to_owned() + location + "]"))
    )
}

impl RunnerVisitorNoBody for Runner {
	fn visit_list(&mut self, it: &mut List) -> Box<dyn Value> {
        create_todo("visit_list")
    }

	fn visit_leaf(&mut self, it: &mut Leaf) -> Box<dyn Value> {
        match &it.value {
            Token::Operator { value } => StringValue::create(&value),
            Token::Delimiter { value } => StringValue::create(&value),
            Token::String { value } => {
                match &**value {
                    "None" => NoneValue::create(),
                    "True" => BooleanValue::create(true),
                    "False" => BooleanValue::create(false),
                    _ => StringValue::create(&value)
                }
            },
            Token::Number { value, base } => {
                let mut result = 0;
                let mut shift = 1;

                for that in value.bytes().rev() {
                    result += ((that - '0' as u8) as i32) * shift;
                    shift *= *base as i32;
                }

                NumberValue::create(result)
            },
            _ => NoneValue::create()
        }
    }

    fn visit_command(&mut self, it: &mut Command) -> Box<dyn Value> {
        for that in &mut it.arguments {
            let resolved = that.accept_runner_visitor_no_body(self);
            self.last_command.push(resolved);
        }

        Box::new(NoneValue::new())
    }

    fn visit_pipeline(&mut self, it: &mut Pipeline) -> Box<dyn Value> {
        let mut commands = vec![];
        let mut first: Option<Box<dyn Value>> = None;

        for that in &mut it.commands {
            let mut command = with_new_command! { self =>
                let _ = that.accept_runner_visitor_no_body(self)
            };

            let mut arguments = vec![];

            for value in &command {
                arguments.push(value.to_string());
            }

            if let None = first {
                if !command.is_empty() {
                    first = Some(command.remove(0));
                }
            }

            if !arguments.is_empty() && arguments[0] == "exit" {
                self.should_exit = true;
                return NoneValue::create();
            }

            if let Some(thing) = &first {
                if thing.to_string() == "pass" {
                    if !command.is_empty() {
                        return command.remove(0);
                    }

                    return NoneValue::create();
                }
            }

            commands.push(arguments);
        }

        if commands.is_empty() || commands[0].is_empty() {
            return NoneValue::create();
        }

        if let Some(first) = first {
            if let Some(_) = cast!(first => StringValue) {
                println!("PIPE: {:?}", commands);

                let maybe_child = launch_pipeline::<fs::File, fs::File>(None, None, &commands);

                if let Ok(child) = maybe_child {
                    println!("child ok");
                    let maybe_result = child.wait_with_output();

                    if let Ok(result) = maybe_result {
                        println!("result ok");
                        let output = String::from_utf8_lossy(&result.stdout);
                        println!("output: {:?}", output);
                        return Box::new(StringValue::new("DONE"));
                    }

                    println!("couln't capture output");
                } else {
                    println!("couln't spawn a child");
                }
            } else {
                return first;
            }
        }

        create_todo("visit_pipeline")
    }

    fn visit_unary(&mut self, it: &mut Unary) -> Box<dyn Value> {
        let operator = it.operator.accept_runner_visitor_no_body(self);
        let target = it.target.accept_runner_visitor_no_body(self);

        match &*operator.to_string() {
            "+" => target.unary_plus(),
            "-" => target.unary_minus(),
            "not" => target.not(),
            "$" => StringValue::create("[getter]"),
            "@" => StringValue::create("[descriptor]"),
            _ => NoneValue::create()
        }
    }

    fn visit_binary(&mut self, it: &mut Binary) -> Box<dyn Value> {
        let operator = it.operator.accept_runner_visitor_no_body(self);
        let lefter = it.lefter.accept_runner_visitor_no_body(self);
        let righter = it.righter.accept_runner_visitor_no_body(self);

        match &*operator.to_string() {
            "+" => lefter.plus(righter),
            "-" => lefter.minus(righter),
            "*" => lefter.times(righter),
            "/" => lefter.divide(righter),
            "^" => lefter.power(righter),
            _ => NoneValue::create()
        }
    }

    fn visit_expressions(&mut self, it: &mut Expressions) -> Box<dyn Value> {
        let mut last: Option<Box<dyn Value>> = None;

        for that in &mut it.values {
            last = Some(that.accept_runner_visitor_no_body(self));
        }

        if let Some(thing) = last {
            thing
        } else {
            NoneValue::create()
        }
    }

    fn visit_closure(&mut self, it: &mut Closure) -> Box<dyn Value> {
        create_todo("visit_closure")
    }

    fn visit_file(&mut self, it: &mut File) -> Box<dyn Value> {
        create_todo("visit_file")
    }
}
