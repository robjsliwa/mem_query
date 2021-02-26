#[macro_export]
macro_rules! doc {
  ($($json:tt)+) => {
    // serde_json::json!({
    //   "_id": 1,
    //   // ...$($json)+
    // })
    {
    let mut v = serde_json::json!($($json)+);
    v["_id"] = serde_json::json!(1);
    v
    }
  };
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_adding_id() {
    let doc = doc!({ "name": "test", "value": 1 });
    assert_eq!(doc["_id"], 1);
    println!("doc {}", doc);
  }
}
