mod doc;
mod errors;

use crate::errors::Error;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

const EQ: &str = "$eq";
const GT: &str = "$gt";
const GTE: &str = "$gte";
const LT: &str = "$lt";
const LTE: &str = "$lte";
const NE: &str = "$ne";
const IN: &str = "$in";
const NIN: &str = "$nin";

const AND: &str = "$and";
const OR: &str = "$or";

fn is_comparison_op(key: &str) -> bool {
  match key {
    EQ | GT | GTE | LT | LTE | NE | IN | NIN => true,
    _ => false,
  }
}

fn is_logical_op(key: &str) -> bool {
  match key {
    AND | OR => true,
    _ => false,
  }
}

fn all(logic_list: Vec<bool>) -> bool {
  logic_list.iter().all(|b| *b)
}

fn any(logic_list: Vec<bool>) -> bool {
  logic_list.iter().any(|b| *b)
}

type DocumentCollection = Arc<Mutex<Vec<Value>>>;
pub type Documents = Vec<Value>;

pub fn make_documents() -> Documents {
  Vec::new()
}

#[derive(Clone)]
struct Collection {
  pub data: DocumentCollection,
}

impl Collection {
  pub fn new() -> Collection {
    Collection {
      data: Arc::new(Mutex::new(Vec::new())),
    }
  }

  pub fn insert(&self, document: Value) {
    self.data.lock().unwrap().push(document);
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

  fn get_collection(&self, collection_name: &str) -> Result<Collection, Error> {
    match self.collections.lock().unwrap().get(collection_name) {
      Some(c) => Ok(c.clone()),
      None => Err(Error::MQCollectionNotFound),
    }
  }

  pub fn insert(&self, collection_name: &str, document: Value) -> Result<(), Error> {
    if !document.is_object() {
      return Err(Error::MQError(String::from(
        "Document must be a JSON object.",
      )));
    }

    self.get_collection(collection_name)?.insert(document);

    Ok(())
  }

  pub fn find(&self, collection_name: &str, query: Value) -> Result<Documents, Error> {
    if !query.is_object() {
      return Err(Error::MQError(String::from("Query must be a JSON object.")));
    }

    let collection = self.get_collection(collection_name)?;

    let mut result: Documents = make_documents();

    for document in collection.data.lock().unwrap().iter() {
      if self.perform_query(&query, document)? {
        result.push(document.clone());
      }
    }

    Ok(result)
  }

  fn perform_query(&self, query: &Value, document: &Value) -> Result<bool, Error> {
    let query_obj = query.as_object().unwrap();
    let mut is_found = false;
    let mut compare_value = &Value::Null;

    for key in query_obj.keys() {
      let key_parts: Vec<&str> = key.split('.').collect();
      if key_parts.len() == 1 {
        if is_logical_op(key) {
          let query_list = &query[key];
          is_found = self.perform_logical_op(key, query_list, document)?;
          break;
        } else if is_comparison_op(key) {
          // TODO
        } else {
          compare_value = &document[key];
        }
      } else {
        // TODO: nested objects
      }

      if &query[key] == compare_value {
        is_found = true;
      } else {
        is_found = false;
        break;
      }
    }

    Ok(is_found)
  }

  fn perform_logical_op(&self, op: &str, query: &Value, document: &Value) -> Result<bool, Error> {
    let query_list = match query.as_array() {
      Some(l) => l,
      None => return Err(Error::MQError(String::from("Logical operation"))),
    };

    let mut op_success_list: Vec<bool> = Vec::new();

    for query in query_list {
      op_success_list.push(self.perform_query(query, document)?);
    }
    Ok(match op {
      OR => any(op_success_list),
      AND => all(op_success_list),
      _ => false,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn verify_operator_generation() {
    assert_eq!(EQ, "$eq");
  }

  #[test]
  fn test_create_collection() -> Result<(), Error> {
    let memdb = MemDb::new();
    memdb.create_collection("TestCollection");
    let _ = memdb.get_collection("TestCollection")?;
    Ok(())
  }

  #[test]
  fn test_simple_query() -> Result<(), Error> {
    let memdb = MemDb::new();
    memdb.create_collection("TestCollection");
    memdb.insert("TestCollection", doc!({ "name": "Rob", "age": 25 }))?;
    memdb.insert("TestCollection", doc!({ "name": "Bob", "age": 20 }))?;
    memdb.insert("TestCollection", doc!({ "name": "Tom", "age": 30 }))?;

    let docs = memdb.find("TestCollection", query!({"name": "Bob"}))?;

    assert_eq!(docs.len(), 1);
    assert_eq!(docs[0]["name"], "Bob");
    Ok(())
  }

  #[test]
  fn test_simple_query_with_multiple_conditions() -> Result<(), Error> {
    let memdb = MemDb::new();
    memdb.create_collection("TestCollection");
    memdb.insert("TestCollection", doc!({ "name": "Rob", "age": 25 }))?;
    memdb.insert("TestCollection", doc!({ "name": "Bob", "age": 20 }))?;
    memdb.insert("TestCollection", doc!({ "name": "Tom", "age": 30 }))?;

    let docs = memdb.find("TestCollection", query!({"name": "Bob", "age": 20}))?;

    assert_eq!(docs.len(), 1);
    assert_eq!(docs[0]["name"], "Bob");
    Ok(())
  }

  #[test]
  fn test_nomatch_query_with_multiple_conditions() -> Result<(), Error> {
    let memdb = MemDb::new();
    memdb.create_collection("TestCollection");
    memdb.insert("TestCollection", doc!({ "name": "Rob", "age": 25 }))?;
    memdb.insert("TestCollection", doc!({ "name": "Bob", "age": 20 }))?;
    memdb.insert("TestCollection", doc!({ "name": "Tom", "age": 30 }))?;

    let docs = memdb.find("TestCollection", query!({"name": "Bob", "age": 21}))?;

    assert_eq!(docs.len(), 0);
    Ok(())
  }

  #[test]
  fn test_query_match_with_and() -> Result<(), Error> {
    let memdb = MemDb::new();
    memdb.create_collection("TestCollection");
    memdb.insert("TestCollection", doc!({ "name": "Rob", "age": 25 }))?;
    memdb.insert("TestCollection", doc!({ "name": "Bob", "age": 20 }))?;
    memdb.insert("TestCollection", doc!({ "name": "Tom", "age": 30 }))?;

    let docs = memdb.find(
      "TestCollection",
      query!({ "$and": [{ "name": "Bob" }, { "age": 20 }] }),
    )?;

    assert_eq!(docs.len(), 1);
    assert_eq!(docs[0]["name"], "Bob");
    Ok(())
  }

  #[test]
  fn test_query_nomatch_with_and() -> Result<(), Error> {
    let memdb = MemDb::new();
    memdb.create_collection("TestCollection");
    memdb.insert("TestCollection", doc!({ "name": "Rob", "age": 25 }))?;
    memdb.insert("TestCollection", doc!({ "name": "Bob", "age": 20 }))?;
    memdb.insert("TestCollection", doc!({ "name": "Tom", "age": 30 }))?;

    let docs = memdb.find(
      "TestCollection",
      query!({ "$and": [{ "name": "Bob" }, { "age": 21 }] }),
    )?;

    assert_eq!(docs.len(), 0);
    Ok(())
  }

  #[test]
  fn test_query_match_with_or() -> Result<(), Error> {
    let memdb = MemDb::new();
    memdb.create_collection("TestCollection");
    memdb.insert("TestCollection", doc!({ "name": "Rob", "age": 25 }))?;
    memdb.insert("TestCollection", doc!({ "name": "Bob", "age": 20 }))?;
    memdb.insert("TestCollection", doc!({ "name": "Tom", "age": 30 }))?;

    let docs = memdb.find(
      "TestCollection",
      query!({ "$or": [{ "name": "Bob" }, { "age": 30 }] }),
    )?;

    assert_eq!(docs.len(), 2);
    Ok(())
  }

  #[test]
  fn test_query_nomatch_with_or() -> Result<(), Error> {
    let memdb = MemDb::new();
    memdb.create_collection("TestCollection");
    memdb.insert("TestCollection", doc!({ "name": "Rob", "age": 25 }))?;
    memdb.insert("TestCollection", doc!({ "name": "Bob", "age": 20 }))?;
    memdb.insert("TestCollection", doc!({ "name": "Tom", "age": 30 }))?;

    let docs = memdb.find(
      "TestCollection",
      query!({ "$or": [{ "name": "Toby" }, { "age": 40 }] }),
    )?;

    assert_eq!(docs.len(), 0);
    Ok(())
  }
}
