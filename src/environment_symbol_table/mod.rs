pub mod types;

use types::Value;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;


#[derive(Clone, Debug)]
pub struct Environment {
    pub values: RefCell<HashMap<String, Value>>,
    pub parent: Option<Rc<Environment>>,
}


impl Environment {
    pub fn new() -> Self {
        Self {
            values: RefCell::new(HashMap::new()),
            parent: None,
        }
    }

    pub fn with_enclosing(parent: Rc<Environment>) -> Self {
        Self {
            values: RefCell::new(HashMap::new()),
            parent: Some(parent),
        }
    }

    pub fn define(&self, name: &str, value: Value) {
        self.values.borrow_mut().insert(name.to_string(), value);
    }

    pub fn assign(&self, name: &str, value: Value) -> Result<(), String> {
        if self.values.borrow_mut().contains_key(name) {
            self.values.borrow_mut().insert(name.to_string(), value);
            return Ok(());
        }

        if let Some(ref parent) = self.parent {
            return parent.assign(name, value);
        }

        Err(format!("Undefined variable '{}'", name))
    }

    pub fn get(&self, name: &str) -> Result<Value, String> {
        if let Some(val) = self.values.borrow().get(name) {
            return Ok(val.clone());
        }

        if let Some(ref parent) = self.parent {
            return parent.get(name);
        }

        Err(format!("Undefined variable '{}'", name))
    }
}
