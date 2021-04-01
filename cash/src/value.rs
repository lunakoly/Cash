pub mod string;
pub mod number;
pub mod none;

use std::fmt::Debug;

use std::any::Any;

pub trait Value : Debug {
    fn as_any(&self) -> &dyn Any;

    fn to_string(&self) -> String;

    fn get(&self, subscripts: &[Box<dyn Value>]) -> Box<dyn Value>;
    fn set(&self, subscripts: &[Box<dyn Value>], value: Box<dyn Value>);

    fn unary_plus(&self) -> Box<dyn Value>;
    fn unary_minus(&self) -> Box<dyn Value>;
    fn not(&self) -> Box<dyn Value>;

    fn power(&self, other: Box<dyn Value>) -> Box<dyn Value>;

    fn times(&self, other: Box<dyn Value>) -> Box<dyn Value>;
    fn divide(&self, other: Box<dyn Value>) -> Box<dyn Value>;
    fn reminder(&self, other: Box<dyn Value>) -> Box<dyn Value>;

    fn plus(&self, other: Box<dyn Value>) -> Box<dyn Value>;
    fn minus(&self, other: Box<dyn Value>) -> Box<dyn Value>;

    fn contains(&self, other: Box<dyn Value>) -> bool;
    fn equals(&self, other: Box<dyn Value>) -> bool;

    fn compare(&self, other: Box<dyn Value>) -> i8;
}
