use crate::value::*;
use crate::value::none::NoneValue;
use crate::value::number::NumberValue;
use crate::value::boolean::BooleanValue;
use crate::value::scope::ScopeData;

use frontend::ast::*;

use std::rc::Rc;
use std::cell::RefCell;

pub struct ClosureData {
    pub arguments: Box<dyn Node>,
    pub body: Box<dyn Node>,
    pub scope: Rc<RefCell<ScopeData>>,
}

pub const CLOSURE_TYPE: &'static str = "Closure";

pub struct ClosureValue {
    pub data: Rc<RefCell<ClosureData>>,
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
        data: Rc<RefCell<ClosureData>>
    ) -> ClosureValue {
        ClosureValue {
            data: data,
        }
    }

    pub fn create(
        data: Rc<RefCell<ClosureData>>
    ) -> Box<ClosureValue> {
        Box::new(ClosureValue::new(data))
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

    fn duplicate_or_move(&mut self) -> Box<dyn Value> {
        ClosureValue::create(self.data.clone())
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
