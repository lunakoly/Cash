include!(concat!(env!("OUT_DIR"), "/ast.rs"));

use orders::stream::analyzable_stream::*;
use orders::stream::accumulator_stream::*;
use orders::stream::buffered_stream::*;
use orders::stream::text_stream::*;
use orders::stream::Stream;

use orders::{within, within_parentheses, parse_binary, parse_list};

use nodes::*;

#[allow(dead_code)]
struct Context {
    input: SimpleAnalyzableStream,
    indent_level: u32,
    line_number: u32,
}

fn is_non_operator(symbol: char) -> bool {
    return
        ('a'..'z').contains(&symbol) ||
        ('A'..'Z').contains(&symbol) ||
        symbol == '_';
}

impl Context {
    fn skip_blank(&mut self) {
        while let Some(symbol) = self.input.peek() {
            if
                symbol == '\n' ||
                symbol == '\t' ||
                symbol == ' ' {
                self.input.step();
            } else {
                break;
            }
        }
    }

    fn expect_operator(&mut self, operator: &str) -> bool {
        self.skip_blank();
        self.input.clear();

        let matching_count = self.input.match_text(operator);

        if matching_count == operator.len() {
            self.input.step_all(matching_count);
            return true;
        }

        return false;
    }

    fn expect_keyword(&mut self, keyword: &str) -> bool {
        self.skip_blank();
        self.input.clear();

        let matching_count = self.input.match_text(keyword);

        if let Some(end) = self.input.lookahead(keyword.len()) {
            if !is_non_operator(end) && matching_count > 0 {
                self.input.step_all(matching_count);
                return true;
            }
        }

        return false;
    }

    fn parse_error(&mut self) -> Box<Leaf> {
        self.skip_blank();
        self.input.clear();

        while
            self.input.has_next() &&
            self.input.peek() != Some('\t') &&
            self.input.peek() != Some('\n') &&
            self.input.peek() != Some(' ') {
            self.input.step();
        }

        return Box::new(Leaf {
            value: "<!".to_owned() + &self.input.revise_all() + "!>"
        });
    }

    fn parse_identifier(&mut self) -> Box<Leaf> {
        self.skip_blank();
        self.input.clear();

        while
            self.input.expect_in('0', '9') ||
            self.input.expect_in('a', 'z') ||
            self.input.expect_in('A', 'Z') ||
            self.input.peek() == Some('_') {
            self.input.step();
        }

        let mut name = self.input.revise_all();

        if self.expect_operator(".") {
            name += ".";
            name += &self.parse_identifier().value;
        }

        return Box::new(Leaf {
            value: name
        });
    }

    fn parse_string(&mut self) -> Box<Leaf> {
        self.skip_blank();
        self.input.clear();

        self.input.accept('"');

        while
            self.input.has_next() &&
            !self.input.accept('"') {
            self.input.step();
        }

        return Box::new(Leaf {
            value: self.input.revise_all()
        });
    }

    fn parse_terminal(&mut self) -> Box<dyn Node> {
        within_parentheses! { self =>
            self.parse_plus_minus()
        };

        let it = if self.input.peek() == Some('"') {
            self.parse_string()
        } else {
            self.parse_identifier()
        };

        within_parentheses! { self =>
            Box::new(Call {
                operator: it,
                arguments: self.parse_dot_identifier_equals_expression_list(),
            })
        };

        return it;
    }

    fn parse_bitwise_and(&mut self) -> Box<dyn Node> {
        parse_binary! { self, parse_terminal =>
            self.expect_operator("&")
        };
    }

    fn parse_bitwise_or(&mut self) -> Box<dyn Node> {
        parse_binary! { self, parse_bitwise_and =>
            self.expect_operator("|")
        };
    }

    fn parse_term(&mut self) -> Box<dyn Node> {
        parse_binary! { self, parse_bitwise_or =>
            self.expect_operator("*") ||
            self.expect_operator("/")
        };
    }

    fn parse_plus_minus(&mut self) -> Box<dyn Node> {
        parse_binary! { self, parse_term =>
            self.expect_operator("+") ||
            self.expect_operator("-")
        };
    }

    fn parse_expression(&mut self) -> Box<dyn Node> {
        return self.parse_plus_minus();
    }

    fn parse_identifier_equals_expression(&mut self) -> Box<NamedValue> {
        let name = self.parse_identifier().value.clone();

        let value = if self.expect_operator("=") {
            self.parse_expression()
        } else {
            self.parse_error()
        };

        return Box::new(NamedValue {
            name: name,
            value: value
        });
    }

    fn parse_dot_identifier_equals_expression(&mut self) -> Box<dyn Node> {
        if self.expect_operator(".") {
            return self.parse_identifier_equals_expression();
        }

        return self.parse_expression();
    }

    fn parse_dot_identifier_equals_expression_list(&mut self) -> Vec<Box<dyn Node>> {
        parse_list! { self, parse_dot_identifier_equals_expression, ")" }
    }

    fn parse_module_modifier(&mut self) -> Box<dyn Node> {
        if self.expect_keyword("parameter") {
            return self.parse_identifier_equals_expression();
        }

        return self.parse_error();
    }

    fn parse_port_attribute(&mut self) -> Box<dyn Node> {
        return self.parse_identifier_equals_expression();
    }

    fn parse_variable(&mut self, modifiers: Vec<Box<dyn Node>>) -> Box<dyn Node> {
        let mut it = Variable {
            modifiers: modifiers,
            name: self.parse_identifier().value.clone(),
            proto: None,
            value: None
        };

        if self.expect_operator(":") {
            it.proto = Some(self.parse_expression());
        }

        if self.expect_operator("=") {
            it.value = Some(self.parse_expression());
        }

        return Box::new(it);
    }

    fn parse_module_input(&mut self) -> Box<dyn Node> {
        let mut modifiers = vec![];

        while self.expect_operator("@") {
            modifiers.push(self.parse_port_attribute());
        }

        return self.parse_variable(modifiers);
    }

    fn parse_module_inputs(&mut self) -> Vec<Box<dyn Node>> {
        parse_list! { self, parse_module_input, ")" }
    }

    fn parse_module_level_modifier(&mut self) -> Box<dyn Node> {
        return self.parse_dot_identifier_equals_expression();
    }

    fn parse_module_level(&mut self) -> Box<dyn Node> {
        let mut modifiers = vec![];

        while self.expect_operator("#") {
            modifiers.push(self.parse_module_level_modifier());
        }

        if self.expect_keyword("let") {
            return self.parse_variable(modifiers);
        }

        if self.expect_keyword("always") {
            return self.parse_error();
        }

        return self.parse_error();
    }

    fn parse_module(&mut self, modifiers: Vec<Box<dyn Node>>) -> Box<Module> {
        let mut it = Module {
            declarations: vec![],
            name: self.parse_identifier().value.clone(),
            inputs: vec![],
            modifiers: modifiers
        };

        if self.expect_operator("(") {
            it.inputs = self.parse_module_inputs();
            self.expect_operator(")");
        }

        if self.expect_operator("{") {
            while self.input.has_next() {
                it.declarations.push(self.parse_module_level());
            }
            self.expect_operator("}");
        }

        return Box::new(it);
    }

    fn parse_top_level(&mut self) -> Box<dyn Node> {
        let mut modifiers = vec![];

        while self.expect_operator("#") {
            modifiers.push(self.parse_module_modifier());
        }

        if self.expect_keyword("module") {
            return self.parse_module(modifiers);
        }

        return self.parse_error();
    }

    fn parse_file(&mut self) -> Box<File> {
        let mut it = File {
            declarations: vec![]
        };

        while self.input.has_next() {
            it.declarations.push(self.parse_top_level());
        }

        return Box::new(it);
    }
}

pub fn parse() -> Box<dyn Node> {
    let mut context = Context {
        input: SimpleAnalyzableStream::acquire(16, 5),
        indent_level: 0,
        line_number: 1,
    };

    return context.parse_file();
}
