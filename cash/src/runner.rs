use crate::ast::*;
use crate::ast::nodes::*;

use crate::value::Value;
use crate::value::string::StringValue;
use crate::value::none::NoneValue;

use parsing::ruler::RepresentableToken;

use processing::launch_pipeline;

use std::fs;

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
        let mut value = "".to_owned();

        if let Some(thing) = it.value.get_value() {
            value = thing.to_string();
        }

        println!("LEAF: {:?}", value);
        Box::new(StringValue::new(&value))
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

        for that in &mut it.commands {
            let command = with_new_command! { self =>
                let _ = that.accept_runner_visitor_no_body(self);
            };

            let mut arguments = vec![];

            for value in &command {
                arguments.push(value.to_string());
            }

            if !arguments.is_empty() && arguments[0] == "exit" {
                self.should_exit = true;
            }

            commands.push(arguments);
        }

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

        create_todo("visit_pipeline")
    }

    fn visit_unary(&mut self, it: &mut Unary) -> Box<dyn Value> {
        create_todo("visit_unary")
    }

    fn visit_binary(&mut self, it: &mut Binary) -> Box<dyn Value> {
        create_todo("visit_binary")
    }

    fn visit_expressions(&mut self, it: &mut Expressions) -> Box<dyn Value> {
        for that in &mut it.values {
            that.accept_runner_visitor_no_body(self);
        }

        create_todo("visit_expresisons")
    }

    fn visit_closure(&mut self, it: &mut Closure) -> Box<dyn Value> {
        create_todo("visit_closure")
    }

    fn visit_file(&mut self, it: &mut File) -> Box<dyn Value> {
        create_todo("visit_file")
    }
}
