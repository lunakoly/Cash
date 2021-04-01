use crate::value::*;

#[derive(Clone, Debug)]
pub struct StringValue {
    value: String,
}

impl StringValue {
    pub fn new(value: &str) -> StringValue {
        StringValue {
            value: value.to_owned(),
        }
    }

    fn todo_binary(&self, other: Box<dyn Value>) -> Box<dyn Value> {
        return Box::new(
            StringValue {
                value: self.value.clone() + &other.to_string()
            }
        );
    }
}

impl Value for StringValue {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_string(&self) -> String {
        return self.value.clone();
    }

    fn get(&self, subscripts: &[Box<dyn Value>]) -> Box<dyn Value> {
        return Box::new(
            StringValue {
                value: subscripts.iter().map(|it| it.to_string()).collect(),
            }
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

    fn contains(&self, other: Box<dyn Value>) -> bool {
        return self.value.contains(&other.to_string());
    }

    fn equals(&self, other: Box<dyn Value>) -> bool {
        return self.value == other.to_string();
    }

    fn compare(&self, other: Box<dyn Value>) -> i8 {
        match self.value.len() - other.to_string().len() {
            0 => 0,
            it if it > 0 => 1,
            _ => -1,
        }
    }
}
