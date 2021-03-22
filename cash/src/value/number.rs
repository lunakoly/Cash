use crate::value::*;

use crate::value::string::{StringValue};

#[derive(Clone, Debug)]
pub struct NumberValue {
    value: i32,
}

impl NumberValue {
    pub fn new(value: i32) -> NumberValue {
        NumberValue {
            value: value,
        }
    }

    fn todo_binary(&self, other: Box<dyn Value>) -> Box<dyn Value> {
        return Box::new(
            StringValue::new(
                self.to_string() + &other.to_string()
            )
        );
    }
}

impl Value for NumberValue {
    fn to_string(&self) -> String {
        return self.value.to_string();
    }

    fn get(&self, subscripts: &[Box<dyn Value>]) -> Box<dyn Value> {
        return Box::new(
            StringValue::new(
                subscripts.iter().map(|it| it.to_string()).collect(),
            )
        );
    }

    fn set(&self, _subscripts: &[Box<dyn Value>], _value: Box<dyn Value>) {
        // TODO
    }

    fn unary_plus(&self) -> Box<dyn Value> {
        // TODO
        return Box::new(self.clone());
    }

    fn unary_minus(&self) -> Box<dyn Value> {
        // TODO
        return Box::new(self.clone());
    }

    fn not(&self) -> Box<dyn Value> {
        // TODO
        return Box::new(self.clone());
    }

    fn power(&self, other: Box<dyn Value>) -> Box<dyn Value> {
        self.todo_binary(other)
    }

    fn times(&self, other: Box<dyn Value>) -> Box<dyn Value> {
        self.todo_binary(other)
    }

    fn divide(&self, other: Box<dyn Value>) -> Box<dyn Value> {
        self.todo_binary(other)
    }

    fn reminder(&self, other: Box<dyn Value>) -> Box<dyn Value> {
        self.todo_binary(other)
    }

    fn plus(&self, other: Box<dyn Value>) -> Box<dyn Value> {
        self.todo_binary(other)
    }

    fn minus(&self, other: Box<dyn Value>) -> Box<dyn Value> {
        self.todo_binary(other)
    }

    fn contains(&self, _other: Box<dyn Value>) -> bool {
        return false;
    }

    fn equals(&self, other: Box<dyn Value>) -> bool {
        return self.to_string() == other.to_string();
    }

    fn compare(&self, _other: Box<dyn Value>) -> i8 {
        // TODO
        return 0;
    }
}
