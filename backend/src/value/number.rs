use crate::cast;

use crate::value::*;
use crate::value::none::NoneValue;
use crate::value::boolean::BooleanValue;

pub const NUMBER_TYPE: &'static str = "Number";

#[derive(Clone, Debug)]
pub struct NumberValue {
    pub value: i32,
}

impl NumberValue {
    pub fn new(value: i32) -> NumberValue {
        NumberValue {
            value: value,
        }
    }

    pub fn create(initial: i32) -> Box<NumberValue> {
        Box::new(NumberValue::new(initial))
    }
}

impl Labeled for NumberValue {
    fn get_type_name() -> &'static str {
        NUMBER_TYPE
    }
}

impl Value for NumberValue {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn duplicate_or_move(&mut self) -> Box<dyn Value> {
        NumberValue::create(self.value)
    }

    fn get_type_name(&self) -> &'static str {
        NUMBER_TYPE
    }

    fn to_string(&self) -> String {
        return self.value.to_string();
    }

    fn get(&self, _subscripts: &[Box<dyn Value>]) -> Box<dyn Value> {
        NoneValue::create()
    }

    fn set(&self, _subscripts: &[Box<dyn Value>], _value: Box<dyn Value>) -> Box<dyn Value> {
        NoneValue::create()
    }

    fn unary_plus(&self) -> Box<dyn Value> {
        NumberValue::create(
            self.value
        )
    }

    fn unary_minus(&self) -> Box<dyn Value> {
        NumberValue::create(
            -self.value
        )
    }

    fn not(&self) -> Box<dyn Value> {
        NumberValue::create(
            !self.value
        )
    }

    fn power(&self, other: Box<dyn Value>) -> Box<dyn Value> {
        let maybe_number = cast! { other => NumberValue };

        if let Some(number) = maybe_number {
            let mut result = self.value;

            if number.value < 0 {
                return NumberValue::create(0);
            }

            if number.value == 0 {
                return NumberValue::create(1);
            }

            for _ in 1..number.value {
                result *= self.value;
            }

            return NumberValue::create(result);
        }

        NoneValue::create()
    }

    fn times(&self, other: Box<dyn Value>) -> Box<dyn Value> {
        let maybe_number = cast! { other => NumberValue };

        if let Some(number) = maybe_number {
            return NumberValue::create(self.value * number.value);
        }

        NoneValue::create()
    }

    fn divide(&self, other: Box<dyn Value>) -> Box<dyn Value> {
        let maybe_number = cast! { other => NumberValue };

        if let Some(number) = maybe_number {
            return NumberValue::create(self.value / number.value);
        }

        NoneValue::create()
    }

    fn reminder(&self, other: Box<dyn Value>) -> Box<dyn Value> {
        let maybe_number = cast! { other => NumberValue };

        if let Some(number) = maybe_number {
            return NumberValue::create(self.value % number.value);
        }

        NoneValue::create()
    }

    fn plus(&self, other: Box<dyn Value>) -> Box<dyn Value> {
        let maybe_number = cast! { other => NumberValue };

        if let Some(number) = maybe_number {
            return NumberValue::create(self.value + number.value);
        }

        NoneValue::create()
    }

    fn minus(&self, other: Box<dyn Value>) -> Box<dyn Value> {
        let maybe_number = cast! { other => NumberValue };

        if let Some(number) = maybe_number {
            return NumberValue::create(self.value - number.value);
        }

        NoneValue::create()
    }

    fn contains(&self, _other: Box<dyn Value>) -> Box<BooleanValue> {
        BooleanValue::create(false)
    }

    fn equals(&self, other: Box<dyn Value>) -> Box<BooleanValue> {
        let maybe_number = cast! { other => NumberValue };

        if let Some(number) = maybe_number {
            return BooleanValue::create(self.value == number.value);
        }

        BooleanValue::create(false)
    }

    fn compare(&self, other: Box<dyn Value>) -> Box<NumberValue> {
        let maybe_number = cast! { other => NumberValue };

        if let Some(number) = maybe_number {
            let that = match self.value - number.value {
                0 => 0,
                it if it > 0 => 1,
                _ => -1,
            };

            return NumberValue::create(that);
        }

        NumberValue::create(0)
    }
}
