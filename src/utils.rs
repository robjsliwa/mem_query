use serde_json::Value;

pub const EQ: &str = "$eq";
pub const GT: &str = "$gt";
pub const GTE: &str = "$gte";
pub const LT: &str = "$lt";
pub const LTE: &str = "$lte";
pub const NE: &str = "$ne";
pub const IN: &str = "$in";
pub const NIN: &str = "$nin";

pub const AND: &str = "$and";
pub const OR: &str = "$or";

pub fn has_comparison_op(compare_obj: &Value) -> (bool, &str, &Value) {
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

pub fn is_comparison_op(compare_obj: &Value) -> bool {
  let (is_comp_op, _, _) = has_comparison_op(compare_obj);
  is_comp_op
}

pub fn is_logical_op(key: &str) -> bool {
  match key {
    AND | OR => true,
    _ => false,
  }
}

pub fn is_op(key: &str) -> bool {
  key.starts_with("$")
}

pub fn all(logic_list: Vec<bool>) -> bool {
  logic_list.iter().all(|b| *b)
}

pub fn any(logic_list: Vec<bool>) -> bool {
  logic_list.iter().any(|b| *b)
}

pub fn is_embedded_query(key: &str) -> (bool, Vec<&str>) {
  let key_parts: Vec<&str> = key.split('.').collect();
  let is_embedded = match key_parts.len() {
    1 => false,
    _ => true,
  };
  (is_embedded, key_parts)
}
