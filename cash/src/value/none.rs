use crate::value::*;

#[derive(Clone, Debug)]
pub struct NoneValue {}

impl NoneValue {
    pub fn new() -> NoneValue {
        NoneValue {}
    }

    fn todo_binary(&self, other: Box<dyn Value>) -> Box<dyn Value> {
        Box::new(NoneValue::new())
    }
}

impl Value for NoneValue {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_string(&self) -> String {
        return "None".to_owned()
    }

    fn get(&self, subscripts: &[Box<dyn Value>]) -> Box<dyn Value> {
        Box::new(NoneValue::new())
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

    fn contains(&self, other: Box<dyn Value>) -> bool {
        return false
    }

    fn equals(&self, other: Box<dyn Value>) -> bool {
        return false
    }

    fn compare(&self, other: Box<dyn Value>) -> i8 {
        0
    }
}
