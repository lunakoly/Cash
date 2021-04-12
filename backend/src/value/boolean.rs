use crate::cast;

use crate::value::*;
use crate::value::none::NoneValue;
use crate::value::number::NumberValue;

pub const BOOLEAN_TYPE: &'static str = "Boolean";

#[derive(Clone, Debug)]
pub struct BooleanValue {
    pub value: bool,
}

impl BooleanValue {
    pub fn new(initial: bool) -> BooleanValue {
        BooleanValue {
            value: initial,
        }
    }

    pub fn create(initial: bool) -> Box<BooleanValue> {
        Box::new(BooleanValue::new(initial))
    }
}

impl Labeled for BooleanValue {
    fn get_type_name() -> &'static str {
        BOOLEAN_TYPE
    }
}

impl Value for BooleanValue {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn duplicate_or_move(&mut self) -> Box<dyn Value> {
        BooleanValue::create(self.value)
    }

    fn get_type_name(&self) -> &'static str {
        BOOLEAN_TYPE
    }

    fn to_string(&self) -> String {
        if self.value {
            "True".to_owned()
        } else {
            "False".to_owned()
        }
    }

    fn get(&self, _subscripts: &[Box<dyn Value>]) -> Box<dyn Value> {
        BooleanValue::create(false)
    }

    fn set(&self, _subscripts: &[Box<dyn Value>], _value: Box<dyn Value>) -> Box<dyn Value> {
        NoneValue::create()
    }

    fn unary_plus(&self) -> Box<dyn Value> {
        BooleanValue::create(false)
    }

    fn unary_minus(&self) -> Box<dyn Value> {
        BooleanValue::create(false)
    }

    fn not(&self) -> Box<dyn Value> {
        BooleanValue::create(false)
    }

    fn power(&self, _other: Box<dyn Value>) -> Box<dyn Value> {
        BooleanValue::create(false)
    }

    fn times(&self, _other: Box<dyn Value>) -> Box<dyn Value> {
        BooleanValue::create(false)
    }

    fn divide(&self, _other: Box<dyn Value>) -> Box<dyn Value> {
        BooleanValue::create(false)
    }

    fn reminder(&self, _other: Box<dyn Value>) -> Box<dyn Value> {
        BooleanValue::create(false)
    }

    fn plus(&self, _other: Box<dyn Value>) -> Box<dyn Value> {
        BooleanValue::create(false)
    }

    fn minus(&self, _other: Box<dyn Value>) -> Box<dyn Value> {
        BooleanValue::create(false)
    }

    fn contains(&self, _other: Box<dyn Value>) -> Box<BooleanValue> {
        BooleanValue::create(false)
    }

    fn equals(&self, other: Box<dyn Value>) -> Box<BooleanValue> {
        let maybe_boolean = cast! { other => BooleanValue };

        if let Some(boolean) = maybe_boolean {
            return BooleanValue::create(
                boolean.value == self.value
            );
        }

        return BooleanValue::create(false);
    }

    fn compare(&self, _other: Box<dyn Value>) -> Box<NumberValue> {
        NumberValue::create(0)
    }
}
