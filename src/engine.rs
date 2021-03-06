use super::{errors::Error, utils::*};
use serde_json::Value;

pub type Documents = Vec<Value>;

pub struct Engine<'a> {
  docs: &'a Documents,
}

impl<'a> Engine<'a> {
  pub fn with_collection(docs: &'a Documents) -> Engine<'a> {
    Engine { docs }
  }

  pub fn find(&self, query: &Value) -> Result<Documents, Error> {
    let mut result: Documents = Vec::new();

    for document in self.docs.iter() {
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

  fn get_nested_document_value<'b>(
    &self,
    nested_keys: Vec<&str>,
    document: &'b Value,
  ) -> Result<&'b Value, Error> {
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
