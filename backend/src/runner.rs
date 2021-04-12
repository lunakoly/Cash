use frontend::ast::*;
use frontend::ast::nodes::*;

use crate::value::Value;
use crate::value::none::NoneValue;
use crate::value::number::NumberValue;
use crate::value::boolean::BooleanValue;
use crate::value::string::StringValue;
use crate::value::closure::{ClosureValue, ClosureData};
use crate::value::scope::{ScopeValue, ScopeData};

use processing::launch_pipeline;

use std::fs;

use frontend::lexer::Token;

use crate::{cast, cast_mut};

use helpers::{elvis, result_or};

use std::rc::Rc;
use std::cell::RefCell;

pub struct Runner {
    pub value: Box<dyn Value>,
    pub command: Vec<Box<dyn Value>>,
    pub should_exit: bool,
    pub scope: Box<ScopeValue>,
}

impl Runner {
    pub fn new() -> Runner {
        Runner {
            value: NoneValue::create(),
            command: vec![],
            should_exit: false,
            scope: ScopeValue::create(ScopeData::create_global()),
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

macro_rules! with_scope {
    ( $scope:expr, $this:expr => $visit_call:expr ) => {
        with! { $this.scope => $scope => $visit_call }
    };
}

fn create_todo(location: &str) -> Box<dyn Value> {
    StringValue::create(&("[todo:".to_owned() + location + "]"))
}

impl SimpleVisitor for Runner {
    fn visit_number(&mut self, it: &mut Number) {
        let mut result = 0;
        let mut shift = 1;

        for that in it.value.bytes().rev() {
            result += ((that - '0' as u8) as i32) * shift;
            shift *= it.base as i32;
        }

        self.value = NumberValue::create(result);
    }

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
                let mut data = closure.data.borrow_mut();

                with_scope! { ScopeValue::create(data.scope.clone()), self =>
                    self.value = with_value! { self =>
                        data.body.accept_simple_visitor(self)
                    }
                };
                return;
            }

            if let None = cast!(&command[0] => StringValue) {
                self.value = command.remove(0);
                return;
            }

            if let Some(value) = self.scope.get_value(&command[0].to_string()) {
                self.value = value;
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

    fn visit_assignment(&mut self, it: &mut Assignment) {
        let mut receiver = String::new();

        let mut extractor = Extractor::new(|it: &mut Text| {
            receiver += &it.value;
        });

        it.receiver.accept_simple_visitor(&mut extractor);

        if receiver.is_empty() {
            println!("Warning > Assignment ignored > Receiver is not a valid name");
            self.value = NoneValue::create();
            return;
        }

        self.value = with_value! { self => it.value.accept_simple_visitor(self) };
        self.scope.set_value(&receiver, self.value.duplicate_or_move());
    }

    fn visit_closure_arguments(&mut self, _it: &mut ClosureArguments) {
        self.value = create_todo("closure_arguments")
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
                ClosureArguments {
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

        self.value = ClosureValue::create(
            Rc::new(
                RefCell::new(
                    ClosureData {
                        arguments: arguments,
                        body: body,
                        scope: ScopeData::create(Some(self.scope.data.clone())),
                    }
                )
            )
        );
    }
}
