use crate::value::*;
use crate::value::none::NoneValue;
use crate::value::number::NumberValue;
use crate::value::boolean::BooleanValue;

use frontend::ast::*;

pub const CLOSURE_TYPE: &'static str = "Closure";

pub struct ClosureValue {
    pub arguments: Box<dyn Node>,
    pub body: Box<dyn Node>,
}

// impl Clone for ClosureValue {
//     fn clone(&self) -> Self {
//         ClosureValue::new(
//             Box::new(
//                 List {
//                     values: vec![],
//                 }
//             ),
//             Box::new(
//                 Expressions {
//                     values: vec![],
//                 }
//             )
//         )
//     }
// }

impl Debug for ClosureValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClosureValue")
            .field("arguments", &"[some ast]".to_owned())
            .field("body", &"[some ast]".to_owned())
            .finish()
    }
}

impl ClosureValue {
    pub fn new(
        arguments: Box<dyn Node>,
        body: Box<dyn Node>,
    ) -> ClosureValue {
        ClosureValue {
            arguments: arguments,
            body: body,
        }
    }

    pub fn create(
        arguments: Box<dyn Node>,
        body: Box<dyn Node>,
    ) -> Box<ClosureValue> {
        Box::new(ClosureValue::new(arguments, body))
    }
}

impl Labeled for ClosureValue {
    fn get_type_name() -> &'static str {
        CLOSURE_TYPE
    }
}

impl Value for ClosureValue {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_type_name(&self) -> &'static str {
        CLOSURE_TYPE
    }

    fn to_string(&self) -> String {
        return "[closure]".to_owned();
    }

    fn get(&self, _subscripts: &[Box<dyn Value>]) -> Box<dyn Value> {
        NoneValue::create()
    }

    fn set(&self, _subscripts: &[Box<dyn Value>], _value: Box<dyn Value>) -> Box<dyn Value> {
        NoneValue::create()
    }

    fn unary_plus(&self) -> Box<dyn Value> {
        NoneValue::create()
    }

    fn unary_minus(&self) -> Box<dyn Value> {
        NoneValue::create()
    }

    fn not(&self) -> Box<dyn Value> {
        NoneValue::create()
    }

    fn power(&self, _other: Box<dyn Value>) -> Box<dyn Value> {
        NoneValue::create()
    }

    fn times(&self, _other: Box<dyn Value>) -> Box<dyn Value> {
        NoneValue::create()
    }

    fn divide(&self, _other: Box<dyn Value>) -> Box<dyn Value> {
        NoneValue::create()
    }

    fn reminder(&self, _other: Box<dyn Value>) -> Box<dyn Value> {
        NoneValue::create()
    }

    fn plus(&self, _other: Box<dyn Value>) -> Box<dyn Value> {
        NoneValue::create()
    }

    fn minus(&self, _other: Box<dyn Value>) -> Box<dyn Value> {
        NoneValue::create()
    }

    fn contains(&self, _other: Box<dyn Value>) -> Box<BooleanValue> {
        BooleanValue::create(false)
    }

    fn equals(&self, _other: Box<dyn Value>) -> Box<BooleanValue> {
        BooleanValue::create(false)
    }

    fn compare(&self, _other: Box<dyn Value>) -> Box<NumberValue> {
        NumberValue::create(0)
    }
}
