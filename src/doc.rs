#[macro_export]
macro_rules! doc {
  ($($json:tt)+) => {
    {
    let mut v = serde_json::json!($($json)+);
    assert!(v.is_object());
    v["_id"] = serde_json::json!(uuid::Uuid::new_v4());
    v
    }
  };
}

#[macro_export]
macro_rules! query {
  ($($json:tt)+) => {
    serde_json::json!($($json)+)
  };
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_adding_id() {
    let doc = doc!({ "name": "test", "value": 1 });
    assert_ne!(doc["_id"], serde_json::Value::Null);
    println!("doc {}", doc);
  }

  #[test]
  #[should_panic]
  fn reject_non_object() {
    doc!(1);
  }
}
