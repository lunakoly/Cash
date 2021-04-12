use std::collections::HashMap;

use std::env::vars;

use crate::value::*;
use crate::value::none::NoneValue;
use crate::value::number::NumberValue;
use crate::value::boolean::BooleanValue;
use crate::value::string::StringValue;

use std::rc::Rc;
use std::cell::RefCell;

use helpers::{elvis, some_or};

use crate::{cast_mut};

pub struct ScopeData {
    pub parent: Option<Rc<RefCell<ScopeData>>>,
    pub properties: HashMap<String, Box<dyn Value>>,
}

impl ScopeData {
    pub fn new(
        parent: Option<Rc<RefCell<ScopeData>>>
    ) -> ScopeData {
        ScopeData {
            parent: parent,
            properties: HashMap::new(),
        }
    }

    pub fn create(
        parent: Option<Rc<RefCell<ScopeData>>>
    ) -> Rc<RefCell<ScopeData>> {
        Rc::new(RefCell::new(ScopeData::new(parent)))
    }

    pub fn create_global() -> Rc<RefCell<ScopeData>> {
        let mut data = ScopeData::new(None);

        for (key, value) in vars() {
            data.set_value(&key, StringValue::create(&value));
        }

        return Rc::new(RefCell::new(data));
    }

    pub fn get_value(&mut self, name: &str) -> Option<Box<dyn Value>> {
        if name == "outerScope" {
            return if let Some(real_parent) = self.parent.clone() {
                Some(ScopeValue::create(real_parent))
            } else {
                Some(NoneValue::create())
            }
        }

        if let Some(thing) = self.properties.get_mut(name) {
            return Some(thing.duplicate_or_move());
        }

        if let Some(wrapped) = &self.parent {
            let mut parent = wrapped.borrow_mut();
            return parent.get_value(name);
        }

        return None;
    }

    pub fn set_value(&mut self, name: &str, value: Box<dyn Value>) {
        self.properties.insert(name.to_owned(), value);
    }
}

pub const SCOPE_TYPE: &'static str = "ScopeValue";

pub struct ScopeValue {
    pub data: Rc<RefCell<ScopeData>>,
}

impl ScopeValue {
    pub fn new(
        data: Rc<RefCell<ScopeData>>
    ) -> ScopeValue {
        ScopeValue {
            data: data,
        }
    }

    pub fn create(
        data: Rc<RefCell<ScopeData>>
    ) -> Box<ScopeValue> {
        Box::new(ScopeValue::new(data))
    }

    pub fn get_value(&mut self, name: &str) -> Option<Box<dyn Value>> {
        if name == "this" {
            return Some(ScopeValue::create(self.data.clone()));
        }

        let mut data = self.data.borrow_mut();
        return data.get_value(name);
    }

    pub fn set_value(&mut self, name: &str, value: Box<dyn Value>) {
        let mut data = self.data.borrow_mut();
        data.set_value(name, value);
    }

    pub fn resolve_parts(&mut self, parts: &[String]) -> Option<Box<dyn Value>> {
        if parts.is_empty() {
            return None;
        }

        let mut result = some_or! { self.get_value(&parts[0]) => return None };

        for it in parts.iter().skip(1) {
            if let Some(scope) = cast_mut!(result => ScopeValue) {
                result = some_or! { scope.get_value(it) => return None };
            } else {
                return None;
            }
        }

        return Some(result);
    }

    pub fn resolve(&mut self, qualified_name: &str) -> Option<Box<dyn Value>> {
        let parts = qualified_name.split(".")
            .map(|it| it.to_owned())
            .collect::<Vec<String>>();
        return self.resolve_parts(&parts);
    }
}

impl Debug for ScopeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut data = self.data.borrow_mut();
        let properties = format!("{:?}", data.properties);

        f.debug_struct("ScopeValue")
            .field("parent", &"[some scope]".to_owned())
            .field("properties", &properties)
            .finish()
    }
}

impl Labeled for ScopeValue {
    fn get_type_name() -> &'static str {
        SCOPE_TYPE
    }
}

impl Value for ScopeValue {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn duplicate_or_move(&mut self) -> Box<dyn Value> {
        // ScopeValue::create(Some(self.data))
        NoneValue::create()
    }

    fn get_type_name(&self) -> &'static str {
        SCOPE_TYPE
    }

    fn to_string(&self) -> String {
        return "[scope]".to_owned();
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
