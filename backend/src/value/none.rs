use crate::value::*;
use crate::value::boolean::BooleanValue;
use crate::value::number::NumberValue;

pub const NONE_TYPE: &'static str = "NoneType";

#[derive(Clone, Debug)]
pub struct NoneValue {}

impl NoneValue {
    pub fn new() -> NoneValue {
        NoneValue {}
    }

    pub fn create() -> Box<NoneValue> {
        Box::new(NoneValue::new())
    }
}

impl Labeled for NoneValue {
    fn get_type_name() -> &'static str {
        NONE_TYPE
    }
}

impl Value for NoneValue {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn duplicate_or_move(&mut self) -> Box<dyn Value> {
        NoneValue::create()
    }

    fn get_type_name(&self) -> &'static str {
        NONE_TYPE
    }

    fn to_string(&self) -> String {
        "None".to_owned()
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

    fn equals(&self, other: Box<dyn Value>) -> Box<BooleanValue> {
        BooleanValue::create(
            other.get_type_name() == NONE_TYPE &&
            other.to_string() == "None"
        )
    }

    fn compare(&self, _other: Box<dyn Value>) -> Box<NumberValue> {
        NumberValue::create(0)
    }
}
