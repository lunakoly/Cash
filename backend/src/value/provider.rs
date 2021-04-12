use crate::value::*;
use crate::value::none::NoneValue;
use crate::value::number::NumberValue;
use crate::value::boolean::BooleanValue;

pub const PROVIDER_TYPE: &'static str = "Provider";

#[derive(Debug)]
pub struct ProviderValue {
    pub delegate: Box<dyn Value>,
}

impl ProviderValue {
    pub fn new(
        delegate: Box<dyn Value>
    ) -> ProviderValue {
        ProviderValue {
            delegate: delegate,
        }
    }

    pub fn create(
        delegate: Box<dyn Value>
    ) -> Box<ProviderValue> {
        Box::new(ProviderValue::new(delegate))
    }
}

impl Labeled for ProviderValue {
    fn get_type_name() -> &'static str {
        PROVIDER_TYPE
    }
}

impl Value for ProviderValue {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn duplicate_or_move(&mut self) -> Box<dyn Value> {
        ProviderValue::create(self.delegate.duplicate_or_move())
    }

    fn get_type_name(&self) -> &'static str {
        PROVIDER_TYPE
    }

    fn to_string(&self) -> String {
        return "[value provider]".to_owned();
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
