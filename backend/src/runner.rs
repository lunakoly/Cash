use frontend::ast::*;
use frontend::ast::nodes::*;

use crate::value::Value;
use crate::value::none::NoneValue;
use crate::value::number::NumberValue;
use crate::value::boolean::BooleanValue;
use crate::value::string::StringValue;
use crate::value::closure::ClosureValue;

use processing::launch_pipeline;

use std::fs;

use frontend::lexer::Token;

use crate::{cast, cast_mut};

use helpers::{elvis, result_or};

pub struct Runner {
    pub value: Box<dyn Value>,
    pub command: Vec<Box<dyn Value>>,
    pub should_exit: bool,
}

impl Runner {
    pub fn new() -> Runner {
        Runner {
            value: NoneValue::create(),
            command: vec![],
            should_exit: false,
        }
    }
}

macro_rules! with {
    ( $this:expr => $replacement:expr => $visit_call:expr ) => {
        {
            let old = std::mem::replace(&mut $this, $replacement);
            $visit_call;
            std::mem::replace(&mut $this, old)
        }
    };
}

macro_rules! with_value {
    ( $this:expr => $visit_call:expr ) => {
        with! { $this.value => NoneValue::create() => $visit_call }
    };
}

macro_rules! with_command {
    ( $this:expr => $visit_call:expr ) => {
        with! { $this.command => vec![] => $visit_call }
    };
}

fn create_todo(location: &str) -> Box<dyn Value> {
    StringValue::create(&("[todo:".to_owned() + location + "]"))
}

impl SimpleVisitor for Runner {
    fn visit_text(&mut self, it: &mut Text) {
        self.value = StringValue::create(&it.value);
    }

    fn visit_text_parts(&mut self, it: &mut TextParts) {
        let mut result = String::new();

        for part in &mut it.parts {
            let value = with_value! { self => part.accept_simple_visitor(self) };
            result += &value.to_string();
        }

        self.value = StringValue::create(&result);
    }

	fn visit_list(&mut self, _it: &mut List) {
        self.value = create_todo("visit_list")
    }

	fn visit_leaf(&mut self, it: &mut Leaf) {
        self.value = match &it.value {
            Token::Operator { value } => {
                StringValue::create(&value)
            },
            Token::Delimiter { value } => {
                StringValue::create(&value)
            },
            Token::Text { value } => {
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
            _ => {
                NoneValue::create()
            }
        }
    }

    fn visit_command(&mut self, it: &mut Command) {
        for that in &mut it.arguments {
            let resolved = with_value! { self => that.accept_simple_visitor(self) };
            self.command.push(resolved);
        }
    }

    fn visit_pipeline(&mut self, it: &mut Pipeline) {
        let mut commands = vec![];

        for that in &mut it.commands {
            let mut command = with_command! { self => that.accept_simple_visitor(self) };
            let mut arguments = vec![];

            for value in &command {
                arguments.push(value.to_string());
            }

            if arguments.is_empty() {
                self.value = NoneValue::create();
                println!("Error > Empty command in pipeline");
                return;
            }

            if arguments[0] == "exit" {
                self.should_exit = true;
                self.value = NoneValue::create();
                return;
            }

            if arguments[0] == "pass" {
                self.value = if command.len() >= 2 {
                    command.remove(1)
                } else {
                    NoneValue::create()
                };
                return;
            }

            if let Some(closure) = cast_mut!(&mut command[0] => ClosureValue) {
                self.value = with_value! { self => closure.body.accept_simple_visitor(self) };
                return;
            }

            if let None = cast!(&command[0] => StringValue) {
                self.value = command.remove(0);
                return;
            }

            commands.push(arguments);
        }

        if commands.is_empty() {
            self.value = NoneValue::create();
            return;
        }

        let maybe_child = launch_pipeline::<fs::File, fs::File>(None, None, &commands);

        let child = result_or! { maybe_child => {
            self.value = NoneValue::create();
            println!("Error > Command spawn a child");
            return;
        }};

        let maybe_result = child.wait_with_output();

        let result = result_or! { maybe_result => {
            self.value = NoneValue::create();
            println!("Error > Couln't capture output");
            return;
        }};

        let _output = String::from_utf8_lossy(&result.stdout);
        self.value = NoneValue::create();
    }

    fn visit_unary(&mut self, it: &mut Unary) {
        let operator = with_value! { self => it.operator.accept_simple_visitor(self) };
        let target = with_value! { self => it.target.accept_simple_visitor(self) };

        self.value = match &*operator.to_string() {
            "+" => target.unary_plus(),
            "-" => target.unary_minus(),
            "not" => target.not(),
            "$" => StringValue::create("[getter]"),
            "@" => StringValue::create("[descriptor]"),
            _ => NoneValue::create()
        }
    }

    fn visit_binary(&mut self, it: &mut Binary) {
        let operator = with_value! { self => it.operator.accept_simple_visitor(self) };
        let lefter = with_value! { self => it.lefter.accept_simple_visitor(self) };
        let righter = with_value! { self => it.righter.accept_simple_visitor(self) };

        self.value = match &*operator.to_string() {
            "+" => lefter.plus(righter),
            "-" => lefter.minus(righter),
            "*" => lefter.times(righter),
            "/" => lefter.divide(righter),
            "^" => lefter.power(righter),
            _ => NoneValue::create()
        }
    }

    fn visit_expressions(&mut self, it: &mut Expressions) {
        let mut last: Option<Box<dyn Value>> = None;

        for that in &mut it.values {
            last = Some(with_value! { self => that.accept_simple_visitor(self) });
        }

        self.value = if let Some(thing) = last {
            thing
        } else {
            NoneValue::create()
        }
    }

    fn visit_closure(&mut self, it: &mut Closure) {
        // don't process closure until it's called

        let arguments = std::mem::replace(
            &mut it.arguments,
            Box::new(
                List {
                    values: vec![],
                }
            )
        );

        let body = std::mem::replace(
            &mut it.body,
            Box::new(
                Expressions {
                    values: vec![],
                }
            )
        );

        self.value = ClosureValue::create(arguments, body);
    }
}
