#[macro_use]
extern crate paste;
mod doc;
mod enum_to_str;

use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

enum_to_str! {
  pub enum ComparisonOperators {
    EQ,
    GT,
    GTE,
    LT,
    LTE,
    NE,
    IN,
    NIN,
  }
}

enum_to_str! {
  pub enum LogicalOperators {
    AND,
    OR,
  }
}

struct Collection {
  pub data: Arc<Mutex<Vec<HashMap<String, Value>>>>,
}

impl Collection {
  pub fn new() -> Collection {
    Collection {
      data: Arc::new(Mutex::new(Vec::new())),
    }
  }
}

pub struct MemDb {
  collections: Arc<Mutex<HashMap<String, Collection>>>,
}

impl MemDb {
  pub fn new() -> MemDb {
    MemDb {
      collections: Arc::new(Mutex::new(HashMap::new())),
    }
  }

  pub fn create_collection(&self, name: &str) {
    let new_collection = Collection::new();
    self
      .collections
      .lock()
      .unwrap()
      .insert(name.to_string(), new_collection);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn verify_operator_generation() {
    assert_eq!(ComparisonOperators::EQ.as_operator(), "$eq");
  }
}
