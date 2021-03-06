use super::{errors::Error, utils::*};
use serde_json::{json, Value};
use std::sync::Arc;

#[cfg(feature = "sync")]
use std::sync::Mutex;

#[cfg(not(feature = "sync"))]
use tokio::sync::Mutex;

pub type Documents = Vec<Value>;

pub type DocumentCollection = Arc<Mutex<Documents>>;

enum MathOpType {
  Inc,
  Mul,
}

pub fn has_update_operations(update: &Value) -> Result<bool, Error> {
  let update = update.as_object().unwrap();
  let all_operators = update.keys().map(|k| k.starts_with("$")).all(|o| o == true);
  if !all_operators && update.keys().map(|k| k.starts_with("$")).any(|o| o == true) {
    return Err(Error::MQInvalidOp(String::from(
      "Cannot mix update operators with keys.",
    )));
  }
  Ok(all_operators)
}

fn is_key_valid_op(key: &str) -> Result<(), Error> {
  // if key contains comparison op check if it is valid
  let (is_embedded, _) = is_embedded_query(key);
  if is_embedded && key.contains("$") {
    return Err(Error::MQOpNotAllowedInMultipartKey);
  }

  if is_op(key) && !matches!(key, EQ | GT | GTE | LT | LTE | NE | IN | NIN | AND | OR) {
    return Err(Error::MQInvalidOp(format!("Op {} is not supported.", key)));
  }

  Ok(())
}

fn sum<T>(x: T, y: T) -> T
where
  T: std::ops::Add<Output = T>,
{
  let result: T = x + y;
  result
}

fn mul<T>(x: T, y: T) -> T
where
  T: std::ops::Mul<Output = T>,
{
  let result: T = x * y;
  result
}

pub struct Engine {
  docs: DocumentCollection,
}

impl Engine {
  pub fn with_collection(docs: DocumentCollection) -> Engine {
    Engine { docs }
  }

  #[cfg(feature = "sync")]
  pub fn find(&self, query: &Value) -> Result<Documents, Error> {
    let mut result: Documents = Vec::new();

    for document in self.docs.lock().unwrap().iter() {
      if self.perform_query(&query, document)? {
        result.push(document.clone());
      }
    }
    Ok(result)
  }

  #[cfg(not(feature = "sync"))]
  pub async fn find(&self, query: &Value) -> Result<Documents, Error> {
    let mut result: Documents = Vec::new();

    for document in self.docs.lock().await.iter() {
      if self.perform_query(&query, document)? {
        result.push(document.clone());
      }
    }
    Ok(result)
  }

  #[cfg(not(feature = "sync"))]
  pub async fn find_and_update(&self, query: &Value, update: &Value) -> Result<u64, Error> {
    let mut documents_updated: u64 = 0;
    for document in self.docs.lock().await.iter_mut() {
      if self.perform_query(&query, document)? {
        self.perform_update(update, document)?;
        documents_updated += 1;
      }
    }
    Ok(documents_updated)
  }

  #[cfg(feature = "sync")]
  pub fn find_and_update(&self, query: &Value, update: &Value) -> Result<u64, Error> {
    let mut documents_updated: u64 = 0;
    for document in self.docs.lock().unwrap().iter_mut() {
      if self.perform_query(&query, document)? {
        self.perform_update(update, document)?;
        documents_updated += 1;
      }
    }
    Ok(documents_updated)
  }

  #[cfg(not(feature = "sync"))]
  pub async fn find_and_delete(&self, query: &Value) -> Result<Documents, Error> {
    let mut docs_deleted: Documents = Vec::new();
    let mut docs_guard = self.docs.lock().await;

    docs_guard.retain(|document| {
      if self.perform_query(&query, &document).unwrap_or(true) {
        docs_deleted.push(document.clone());
        return false;
      }
      true
    });
    Ok(docs_deleted)
  }

  #[cfg(feature = "sync")]
  pub fn find_and_delete(&self, query: &Value) -> Result<Documents, Error> {
    let mut docs_deleted: Documents = Vec::new();
    let mut docs_guard = self.docs.lock().unwrap();

    docs_guard.retain(|document| {
      if self.perform_query(&query, &document).unwrap_or(true) {
        docs_deleted.push(document.clone());
        return false;
      }
      true
    });
    Ok(docs_deleted)
  }

  fn perform_update<'d>(
    &self,
    update: &Value,
    document: &'d mut Value,
  ) -> Result<&'d mut Value, Error> {
    if has_update_operations(update)? {
      self.perform_update_operations(update, document)?;
    } else {
      *document = update.clone();
    }

    Ok(document)
  }

  fn perform_update_operations(&self, update: &Value, document: &mut Value) -> Result<(), Error> {
    let update = update.as_object().unwrap();
    for key in update.keys() {
      match key.as_str() {
        SET => self.handle_set(&update[key], document)?,
        UNSET => self.hanlde_unset(&update[key], document)?,
        INC => self.handle_inc(&update[key], document)?,
        MUL => self.handle_mul(&update[key], document)?,
        _ => {
          return Err(Error::MQInvalidOp(format!(
            "{} is invalid update operator.",
            key
          )))
        }
      }
    }

    Ok(())
  }

  fn run_op_on_value<F>(&self, key: &str, document: &mut Value, op: F) -> Result<(), Error>
  where
    F: Fn(&str, &mut Value) -> Result<(), Error>,
  {
    let (is_embedded, key_parts) = is_embedded_query(key);

    if !is_embedded {
      return op(key, document);
    }

    let key_parts_rest = &key_parts[1..];
    self.run_op_on_value(&key_parts_rest.join("."), &mut document[key_parts[0]], op)?;
    Ok(())
  }

  fn handle_set(&self, update: &Value, document: &mut Value) -> Result<(), Error> {
    let update = match update.as_object() {
      Some(u) => u,
      None => {
        return Err(Error::MQInvalidValue(String::from(
          "$set operator value must be JSON object",
        )))
      }
    };

    for (k, v) in update {
      if has_ops(k) {
        return Err(Error::MQOpNotAllowedInMultipartKey);
      }
      let handler = |k: &str, d: &mut Value| {
        d[k] = v.clone();
        Ok(())
      };
      self.run_op_on_value(k, document, handler)?;
    }

    Ok(())
  }

  fn hanlde_unset(&self, update: &Value, document: &mut Value) -> Result<(), Error> {
    let update = match update.as_object() {
      Some(u) => u,
      None => {
        return Err(Error::MQInvalidValue(String::from(
          "$unset operator value must be JSON object",
        )))
      }
    };

    let handler = |k: &str, d: &mut Value| {
      let document = d.as_object_mut().unwrap();
      document.remove(k);
      Ok(())
    };
    for key in update.keys() {
      self.run_op_on_value(key, document, handler)?;
    }

    Ok(())
  }

  fn handle_inc(&self, update: &Value, document: &mut Value) -> Result<(), Error> {
    self.handle_math_ops(update, document, MathOpType::Inc)
  }

  fn handle_mul(&self, update: &Value, document: &mut Value) -> Result<(), Error> {
    self.handle_math_ops(update, document, MathOpType::Mul)
  }

  fn handle_math_ops(
    &self,
    update: &Value,
    document: &mut Value,
    op_type: MathOpType,
  ) -> Result<(), Error> {
    let update = match update.as_object() {
      Some(u) => u,
      None => {
        return Err(Error::MQInvalidValue(String::from(
          "$inc operator value must be JSON object",
        )))
      }
    };

    for (k, v) in update {
      if has_ops(k) {
        return Err(Error::MQOpNotAllowedInMultipartKey);
      }

      if !v.is_number() {
        return Err(Error::MQInvalidType);
      }

      let handler = |k: &str, d: &mut Value| {
        match (&d[k], &v.clone()) {
          (Value::Number(dk), Value::Number(v)) => {
            if let Some(d_f) = dk.as_f64() {
              if let Some(v_f) = v.as_f64() {
                match op_type {
                  MathOpType::Inc => d[k] = json!(sum(d_f, v_f)),
                  MathOpType::Mul => d[k] = json!(mul(d_f, v_f)),
                };
              }
            } else if let Some(d_i) = dk.as_i64() {
              if let Some(v_i) = v.as_i64() {
                match op_type {
                  MathOpType::Inc => d[k] = json!(sum(d_i, v_i)),
                  MathOpType::Mul => d[k] = json!(mul(d_i, v_i)),
                };
              }
            } else if let Some(d_u) = dk.as_u64() {
              if let Some(v_u) = v.as_u64() {
                match op_type {
                  MathOpType::Inc => d[k] = json!(sum(d_u, v_u)),
                  MathOpType::Mul => d[k] = json!(mul(d_u, v_u)),
                };
              }
            }
          }
          _ => {
            return Err(Error::MQInvalidType);
          }
        };

        Ok(())
      };

      self.run_op_on_value(k, document, handler)?;
    }

    Ok(())
  }

  fn get_document_value<'d>(&self, key: &str, document: &'d Value) -> Result<&'d Value, Error> {
    // find if the value should be the immediate value of the key or embedded document
    let (is_embedded, key_parts) = is_embedded_query(key);
    let mut doc_value = &document[key];
    if is_embedded {
      doc_value = self.get_nested_document_value(key_parts, document)?;
    }

    Ok(doc_value)
  }

  fn perform_query(&self, query: &Value, document: &Value) -> Result<bool, Error> {
    let query_obj = query.as_object().unwrap();
    let mut is_found = false;

    if query_obj.keys().len() == 0 {
      return Ok(true);
    }

    for key in query_obj.keys() {
      is_key_valid_op(key)?;
      if is_logical_op(key) {
        let logical_op_list = &query[key];
        is_found = self.perform_logical_op(key, logical_op_list, document)?;
        break;
      } else if is_comparison_op(&query[key]) {
        let (_, op, comp_value) = has_comparison_op(&query[key]);
        is_found =
          self.perform_comparison_op(op, comp_value, self.get_document_value(key, document)?)?;
        continue;
      } else {
        is_found = &query[key] == self.get_document_value(key, document)?;
        if !is_found {
          break;
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
