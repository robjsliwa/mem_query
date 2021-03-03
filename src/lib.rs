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

fn has_comparison_op(compare_obj: &Value) -> (bool, &str, &Value) {
  match compare_obj.as_object() {
    Some(o) => {
      if o.is_empty() || o.keys().len() > 1 {
        return (false, "", &Value::Null);
      }
      for (key, val) in o.iter() {
        return match &key[..] {
          EQ | GT | GTE | LT | LTE | NE | IN | NIN => (true, key, val),
          _ => (false, "", &Value::Null),
        };
      }
      (false, "", &Value::Null)
    }
    None => return (false, "", &Value::Null),
  }
}

fn is_comparison_op(compare_obj: &Value) -> bool {
  let (is_comp_op, _, _) = has_comparison_op(compare_obj);
  is_comp_op
}

fn is_logical_op(key: &str) -> bool {
  match key {
    AND | OR => true,
    _ => false,
  }
}

fn is_op(key: &str) -> bool {
  key.starts_with("$")
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

    for key in query_obj.keys() {
      let key_parts: Vec<&str> = key.split('.').collect();
      if key_parts.len() == 1 {
        if is_logical_op(key) {
          let logical_op_list = &query[key];
          is_found = self.perform_logical_op(key, logical_op_list, document)?;
          break;
        } else if is_comparison_op(&query[key]) {
          let (_, op, comp_value) = has_comparison_op(&query[key]);
          is_found = self.perform_comparison_op(op, comp_value, &document[key])?;
          continue;
        } else if is_op(key) {
          // all valid ops should have been processed, so this must be unsupported op
          return Err(Error::MQInvalidOp(key.to_string()));
        } else {
          is_found = &query[key] == &document[key];
          if !is_found {
            break;
          }
        }
      } else {
        let nested_value = self.get_nested_document_value(key_parts, document)?;
        if is_comparison_op(&query[key]) {
          let (_, op, comp_value) = has_comparison_op(&query[key]);
          is_found = self.perform_comparison_op(op, comp_value, nested_value)?;
          continue;
        } else {
          is_found = &query[key] == nested_value;
          if !is_found {
            break;
          }
        }
      }
    }

    Ok(is_found)
  }

  fn perform_logical_op(
    &self,
    op: &str,
    logical_op_list: &Value,
    document: &Value,
  ) -> Result<bool, Error> {
    let op_list = match logical_op_list.as_array() {
      Some(l) => l,
      None => return Err(Error::MQError(String::from("Logical operation"))),
    };

    let mut op_success_list: Vec<bool> = Vec::new();

    for op_query in op_list {
      op_success_list.push(self.perform_query(op_query, document)?);
    }
    Ok(match op {
      OR => any(op_success_list),
      AND => all(op_success_list),
      _ => false,
    })
  }

  fn perform_comparison_op(
    &self,
    op: &str,
    compare_to_value: &Value,
    doc_value: &Value,
  ) -> Result<bool, Error> {
    match op {
      GT | GTE | LT | LTE | NE | EQ => self.perform_value_compares(op, compare_to_value, doc_value),
      IN | NIN => Ok(false),
      _ => Err(Error::MQInvalidOp(op.to_string())),
    }
  }

  fn perform_value_compares(
    &self,
    op: &str,
    compare_to_value: &Value,
    doc_value: &Value,
  ) -> Result<bool, Error> {
    if compare_to_value.is_object() {
      return Err(Error::MQInvalidValue(format!(
        "{} expects value not array or object.",
        op
      )));
    }

    match (doc_value, compare_to_value) {
      (Value::Number(d), Value::Number(c)) => {
        if let Some(doc_u) = d.as_u64() {
          if let Some(comp_u) = c.as_u64() {
            return self.compare(op, doc_u, comp_u);
          }
        } else if let Some(doc_i) = d.as_i64() {
          if let Some(comp_i) = d.as_i64() {
            return self.compare(op, doc_i, comp_i);
          }
        } else if let Some(doc_f) = d.as_f64() {
          if let Some(comp_f) = d.as_f64() {
            return self.compare(op, doc_f, comp_f);
          }
        }
      }
      (Value::String(d), Value::String(c)) => return self.compare(op, d, c),
      (Value::Array(d), Value::Array(c)) => {
        return self.perform_array_to_array_compare(op, d, c);
      }
      (Value::Array(d), Value::Number(c)) => {
        return self.perform_array_to_value_compare(op, d, &serde_json::json!(c));
      }
      (Value::Array(d), Value::String(c)) => {
        return self.perform_array_to_value_compare(op, d, &serde_json::json!(c))
      }
      _ => return Err(Error::MQInvalidType),
    }

    Err(Error::MQInvalidType)
  }

  fn compare<T: PartialOrd>(&self, op: &str, d: T, c: T) -> Result<bool, Error> {
    Ok(match op {
      GT => d > c,
      GTE => d >= c,
      LT => d < c,
      LTE => d <= c,
      NE => d != c,
      EQ => d == c,
      _ => {
        return Err(Error::MQInvalidOp(format!(
          "{} not supported for compare.",
          op
        )))
      }
    })
  }

  fn perform_array_to_array_compare(
    &self,
    op: &str,
    document_value: &Vec<Value>,
    compare_value: &Vec<Value>,
  ) -> Result<bool, Error> {
    if document_value == compare_value {
      return Ok(true);
    }

    let mut matches: Vec<bool> = Vec::new();
    for elem in document_value {
      let is_match = match op {
        NE => elem != &serde_json::json!(compare_value),
        EQ => elem == &serde_json::json!(compare_value),
        GT | GTE | LT | LTE | _ => {
          return Err(Error::MQInvalidOp(format!(
            "{} is not valid for array comparison.",
            op
          )));
        }
      };
      matches.push(is_match);
    }

    Ok(any(matches))
  }

  fn perform_array_to_value_compare(
    &self,
    op: &str,
    document_value: &Vec<Value>,
    compare_value: &Value,
  ) -> Result<bool, Error> {
    let mut matches: Vec<bool> = Vec::new();
    for elem in document_value {
      let mut is_match = false;
      if !elem.is_array() {
        // do not check nested arrays
        is_match = self.perform_value_compares(op, compare_value, elem)?
      }
      matches.push(is_match);
    }

    Ok(any(matches))
  }

  fn get_nested_document_value<'a>(
    &self,
    nested_keys: Vec<&str>,
    document: &'a Value,
  ) -> Result<&'a Value, Error> {
    let mut current_value: &Value = document;

    for key in nested_keys {
      if is_op(key) {
        return Err(Error::MQInvalidOp(format!(
          "{} operators not allowed in nested paths.",
          key
        )));
      }
      current_value = &current_value[key];
    }

    Ok(current_value)
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
    memdb.insert("TestCollection", doc!({ "name": "Victor", "age": 20 }))?;

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

  #[test]
  fn test_eq_op() -> Result<(), Error> {
    let memdb = MemDb::new();
    memdb.create_collection("TestCollection");
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
    )?;

    let docs = memdb.find("TestCollection", query!({ "qty": { "$eq": 20 } }))?;

    assert_eq!(docs.len(), 2);
    assert_eq!(docs[0]["item"]["name"], "cd");
    assert_eq!(docs[1]["item"]["name"], "mn");

    Ok(())
  }

  #[test]
  fn test_eq_nomatch_op() -> Result<(), Error> {
    let memdb = MemDb::new();
    memdb.create_collection("TestCollection");
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
    )?;

    let docs = memdb.find("TestCollection", query!({ "qty": { "$eq": 200 } }))?;

    assert_eq!(docs.len(), 0);

    Ok(())
  }

  #[test]
  fn test_eq_op_single_entry_embedded_doc() -> Result<(), Error> {
    let memdb = MemDb::new();
    memdb.create_collection("TestCollection");
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
    )?;

    let docs = memdb.find("TestCollection", query!({ "item.name": { "$eq": "ab" } }))?;

    assert_eq!(docs.len(), 1);
    assert_eq!(docs[0]["item"]["name"], "ab");

    Ok(())
  }

  #[test]
  fn test_eq_op_to_match_array_to_array() -> Result<(), Error> {
    let memdb = MemDb::new();
    memdb.create_collection("TestCollection");
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
    )?;

    let docs = memdb.find(
      "TestCollection",
      query!({ "tags": { "$eq": [ "A", "B"  ] } }),
    )?;

    assert_eq!(docs.len(), 2);
    assert_eq!(docs[0]["item"]["name"], "ij");
    assert_eq!(docs[1]["item"]["name"], "mn");

    Ok(())
  }

  #[test]
  fn test_eq_op_to_nomatch_array_to_array() -> Result<(), Error> {
    let memdb = MemDb::new();
    memdb.create_collection("TestCollection");
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
    )?;

    let docs = memdb.find(
      "TestCollection",
      query!({ "tags": { "$eq": [ "C", "D"  ] } }),
    )?;

    assert_eq!(docs.len(), 0);

    Ok(())
  }

  #[test]
  fn test_eq_op_to_match_array_to_value() -> Result<(), Error> {
    let memdb = MemDb::new();
    memdb.create_collection("TestCollection");
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
    )?;

    let docs = memdb.find("TestCollection", query!({ "tags": { "$eq": "B" } }))?;

    assert_eq!(docs.len(), 4);
    assert_eq!(docs[0]["item"]["name"], "ab");
    assert_eq!(docs[1]["item"]["name"], "cd");
    assert_eq!(docs[2]["item"]["name"], "ij");
    assert_eq!(docs[3]["item"]["name"], "xy");

    Ok(())
  }

  #[test]
  fn test_gt_match() -> Result<(), Error> {
    let memdb = MemDb::new();
    memdb.create_collection("TestCollection");
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "ab", "code": "123" }, "qty": 15, "tags": [ "A", "B", "C" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "cd", "code": "123" }, "qty": 20, "tags": [ "B" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "ij", "code": "456" }, "qty": 25, "tags": [ "A", "B" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "xy", "code": "456" }, "qty": 30, "tags": [ "B", "A" ] }),
    )?;
    memdb.insert(
      "TestCollection",
      doc!({ "item": { "name": "mn", "code": "000" }, "qty": 20, "tags": [ [ "A", "B" ], "C" ] }),
    )?;

    let docs = memdb.find("TestCollection", query!({ "qty": { "$gt": 20 } }))?;

    assert_eq!(docs.len(), 2);
    assert_eq!(docs[0]["item"]["name"], "ij");
    assert_eq!(docs[1]["item"]["name"], "xy");

    Ok(())
  }
}
