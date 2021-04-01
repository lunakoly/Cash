use crate::cast;

use crate::value::*;
use crate::value::none::NoneValue;
use crate::value::number::NumberValue;
use crate::value::boolean::BooleanValue;

pub const STRING_TYPE: &'static str = "String";

#[derive(Clone, Debug)]
pub struct StringValue {
    pub value: String,
}

impl StringValue {
    pub fn new(value: &str) -> StringValue {
        StringValue {
            value: value.to_owned(),
        }
    }

    pub fn create(initial: &str) -> Box<StringValue> {
        Box::new(StringValue::new(initial))
    }
}

impl Labeled for StringValue {
    fn get_type_name() -> &'static str {
        STRING_TYPE
    }
}

impl Value for StringValue {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type_name(&self) -> &'static str {
        STRING_TYPE
    }

    fn to_string(&self) -> String {
        return self.value.clone();
    }

    fn get(&self, subscripts: &[Box<dyn Value>]) -> Box<dyn Value> {
        if subscripts.len() != 1 {
            return NoneValue::create();
        }

        let maybe_number_value = cast! { &subscripts[0] => NumberValue };

        if let Some(number_value) = maybe_number_value {
            let mut value = number_value.value;

            if value >= self.value.len() as i32 {
                return NoneValue::create();
            }

            if value < -(self.value.len() as i32) {
                return NoneValue::create();
            }

            if value < 0 {
                value += self.value.len() as i32;
            }

            let it = self.value.chars().skip(value as usize).take(1).next();

            if let Some(that) = it {
                return StringValue::create(
                    &that.to_string()
                );
            }
        }

        return NoneValue::create();
    }

    fn set(&self, _subscripts: &[Box<dyn Value>], _value: Box<dyn Value>) -> Box<dyn Value> {
        NoneValue::create()
    }

    fn unary_plus(&self) -> Box<dyn Value> {
        let concatenated = "+".to_owned() + &self.value;

        StringValue::create(
            &concatenated
        )
    }

    fn unary_minus(&self) -> Box<dyn Value> {
        let concatenated = "-".to_owned() + &self.value;

        StringValue::create(
            &concatenated
        )
    }

    fn not(&self) -> Box<dyn Value> {
        NoneValue::create()
    }

    fn power(&self, _other: Box<dyn Value>) -> Box<dyn Value> {
        NoneValue::create()
    }

    fn times(&self, other: Box<dyn Value>) -> Box<dyn Value> {
        let maybe_number_value = cast! { other => NumberValue };

        if let Some(number_value) = maybe_number_value {
            if number_value.value < 0 {
                return NoneValue::create()
            }

            let mut repeated = self.value.clone();

            for _ in 1..number_value.value {
                repeated += &self.value;
            }

            return StringValue::create(
                &repeated
            );
        }

        return NoneValue::create();
    }

    fn divide(&self, _other: Box<dyn Value>) -> Box<dyn Value> {
        NoneValue::create()
    }

    fn reminder(&self, _other: Box<dyn Value>) -> Box<dyn Value> {
        NoneValue::create()
    }

    fn plus(&self, other: Box<dyn Value>) -> Box<dyn Value> {
        let concatenated = self.value.clone() + &other.to_string();

        return StringValue::create(
            &concatenated
        );
    }

    fn minus(&self, _other: Box<dyn Value>) -> Box<dyn Value> {
        NoneValue::create()
    }

    fn contains(&self, other: Box<dyn Value>) -> Box<BooleanValue> {
        BooleanValue::create(
            self.value.contains(&other.to_string())
        )
    }

    fn equals(&self, other: Box<dyn Value>) -> Box<BooleanValue> {
        BooleanValue::create(
            self.value == other.to_string()
        )
    }

    fn compare(&self, other: Box<dyn Value>) -> Box<NumberValue> {
        let that = match self.value.len() - other.to_string().len() {
            0 => 0,
            it if it > 0 => 1,
            _ => -1,
        };

        NumberValue::create(that)
    }
}
