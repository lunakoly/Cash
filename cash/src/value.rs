pub mod none;
pub mod boolean;
pub mod number;
pub mod string;

use std::fmt::Debug;

use std::any::Any;

use boolean::BooleanValue;
use number::NumberValue;

pub trait Labeled {
    fn get_type_name() -> &'static str;
}

pub trait Value : Debug {
    fn as_any(&self) -> &dyn Any;

    fn get_type_name(&self) -> &'static str;
    fn to_string(&self) -> String;

    fn get(&self, subscripts: &[Box<dyn Value>]) -> Box<dyn Value>;
    fn set(&self, subscripts: &[Box<dyn Value>], value: Box<dyn Value>) -> Box<dyn Value>;

    fn unary_plus(&self) -> Box<dyn Value>;
    fn unary_minus(&self) -> Box<dyn Value>;
    fn not(&self) -> Box<dyn Value>;

    fn power(&self, other: Box<dyn Value>) -> Box<dyn Value>;

    fn times(&self, other: Box<dyn Value>) -> Box<dyn Value>;
    fn divide(&self, other: Box<dyn Value>) -> Box<dyn Value>;
    fn reminder(&self, other: Box<dyn Value>) -> Box<dyn Value>;

    fn plus(&self, other: Box<dyn Value>) -> Box<dyn Value>;
    fn minus(&self, other: Box<dyn Value>) -> Box<dyn Value>;

    fn contains(&self, other: Box<dyn Value>) -> Box<BooleanValue>;
    fn equals(&self, other: Box<dyn Value>) -> Box<BooleanValue>;

    fn compare(&self, other: Box<dyn Value>) -> Box<NumberValue>;
}

#[macro_export]
macro_rules! cast {
    ( $target:expr => $kind:ty ) => {
        {
            $target.as_any().downcast_ref::<$kind>()
        }
    };
}

// pub trait Labelable {
//     fn to<T: 'static + Labeled>(&self) -> Option<&T>;
// }

// impl <V: Value> Labelable for Box<V> {
//     fn to<T: 'static + Labeled>(&self) -> Option<&T> {
//         self.as_any().downcast_ref::<T>()
//     }
// }
