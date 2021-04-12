use frontend::ast::*;
use frontend::ast::nodes::*;

use crate::value::Value;
use crate::value::none::NoneValue;
use crate::value::number::NumberValue;
use crate::value::string::StringValue;
use crate::value::closure::{ClosureValue, ClosureData};
use crate::value::scope::{ScopeValue, ScopeData};
use crate::value::provider::ProviderValue;

use processing::launch_pipeline;

use std::fs;

use crate::{cast, cast_mut};

use helpers::{elvis, result_or, some_or};

use std::rc::Rc;
use std::cell::RefCell;

pub struct Runner {
    pub value: Box<dyn Value>,
    pub command: Vec<Box<dyn Value>>,
    pub should_exit: bool,
    pub scope: Box<ScopeValue>,
    pub closure_arguments: Vec<String>,
}

impl Runner {
    pub fn new() -> Runner {
        Runner {
            value: NoneValue::create(),
            command: vec![],
            should_exit: false,
            scope: ScopeValue::create(ScopeData::create_global()),
            closure_arguments: vec![],
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

macro_rules! with_closure_arguments {
    ( $this:expr => $visit_call:expr ) => {
        with! { $this.closure_arguments => vec![] => $visit_call }
    };
}

macro_rules! extract_text {
    ( $target:expr ) => {
        {
            let mut text: Option<String> = None;

            let mut extractor = Extractor::new(|it: &mut Text| {
                text = Some(it.value.clone());
            });

            $target.accept_simple_visitor(&mut extractor);
            text
        }
    };
}

impl SimpleVisitor for Runner {
    fn visit_number(&mut self, it: &mut Number) {
        let mut result = 0;
        let mut shift = 1;

        if it.base <= 10 {
            for that in it.value.bytes().rev() {
                result += ((that - '0' as u8) as i32) * shift;
                shift *= it.base as i32;
            }
        } else if it.base > 10 {
            for that in it.value.bytes().rev() {
                if ('0'..='9').contains(&(that as char)) {
                    result += ((that - '0' as u8) as i32) * shift;
                } else if ('a'..='z').contains(&(that as char)) {
                    result += ((that - 'a' as u8) as i32 + 10) * shift;
                } else {
                    result += ((that - 'A' as u8) as i32 + 10) * shift;
                }
                shift *= it.base as i32;
            }
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

            if let None = cast!(&command[0] => StringValue) {
                let mut value = command.remove(0);
                if let Some(provider) = cast_mut!(value => ProviderValue) {
                    self.value = std::mem::replace(&mut provider.delegate, NoneValue::create());
                } else {
                    self.value = value;
                }
                return;
            }

            if let Some(mut value) = self.scope.resolve(&command[0].to_string()) {
                if let Some(closure) = cast_mut!(value => ClosureValue) {
                    let mut data = closure.data.borrow_mut();
                    let mut scope = ScopeValue::create(data.scope.clone());

                    let arguments = with_closure_arguments! {
                        self => data.arguments.accept_simple_visitor(self)
                    };

                    let minimum = std::cmp::min(arguments.len(), command.len() - 1);

                    for index in 0..minimum {
                        scope.set_value(&arguments[index], command.remove(1));
                    }

                    for index in minimum..arguments.len() {
                        scope.set_value(&arguments[index], NoneValue::create());
                    }

                    with_scope! { scope, self =>
                        self.value = with_value! { self =>
                            data.body.accept_simple_visitor(self)
                        }
                    };
                } else {
                    self.value = value;
                }
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

    // fn visit_accessor(&mut self, it: &mut Accessor) {
    //     let inner = some_or! { extract_text!(it.inner) => {
    //         println!("Warning > Accessor ignored > Inner accessor is not a valid name");
    //         self.value = NoneValue::create();
    //         return;
    //     }};

    //     let mut receiver = with_value! { self => it.target.accept_simple_visitor(self) };

    //     if let Some(scope) = cast_mut!(receiver => ScopeValue) {
    //         self.value = some_or! { scope.get_value(&inner) => NoneValue::create() };
    //     } else {
    //         println!("Warning > Accessor ignored > Receiver is not a scope > {:?} {:?}", receiver.get_type_name(), receiver.to_string());
    //         self.value = receiver;
    //     }
    // }

    fn visit_provider(&mut self, it: &mut Provider) {
        let receiver_value = with_value! { self => it.target.accept_simple_visitor(self) };
        let receiver = receiver_value.to_string();

        if receiver.is_empty() {
            println!("Warning > Accessing the provider > Receiver name is empty > {:?}", &receiver);
            self.value = NoneValue::create();
            return;
        }

        if let Some(value) = self.scope.resolve(&receiver) {
            if let Some(..) = cast!(value => ClosureValue) {
                self.value = value;
            } else {
                self.value = ProviderValue::create(value);
            }
        } else {
            println!("Warning > Accessing the provider > Unresolved property name > {:?}", &receiver);
        }
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
        let receiver = some_or! { extract_text!(it.receiver) => {
            println!("Warning > Assignment ignored > Receiver is not a valid name");
            self.value = NoneValue::create();
            return;
        }};

        let parts = receiver.split(".")
            .map(|it| it.to_owned())
            .collect::<Vec<String>>();

        let mut prefix = parts.clone();
        let name = prefix.remove(prefix.len() - 1);

        if name.is_empty() {
            println!("Warning > Assignment ignored > Receiver name is empty > {:?}", &receiver);
            self.value = NoneValue::create();
            return;
        }

        let mut receiver_scope = ScopeValue::create(self.scope.data.clone());

        if !prefix.is_empty() {
            if let Some(mut value) = self.scope.resolve_parts(&prefix) {
                if let Some(scope) = cast_mut!(value => ScopeValue) {
                    receiver_scope = ScopeValue::create(scope.data.clone());
                } else {
                    println!("Warning > Assignment ignored > Receiver prefix is not a scope > {:?}", &receiver);
                    self.value = NoneValue::create();
                    return;
                }
            } else {
                println!("Warning > Assignment ignored > Could't resolve the receiver > {:?}", &receiver);
                self.value = NoneValue::create();
                return;
            }
        };

        self.value = with_value! { self => it.value.accept_simple_visitor(self) };
        receiver_scope.set_value(&name, self.value.duplicate_or_move());
    }

    fn visit_closure_arguments(&mut self, it: &mut ClosureArguments) {
        for that in &mut it.values {
            // let value = with_value! { self => that.accept_simple_visitor(self) };

            let receiver = some_or! { extract_text!(that) => {
                println!("Warning > Assignment ignored > Receiver is not a valid name");
                return;
            }};

            self.closure_arguments.push(receiver);
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
