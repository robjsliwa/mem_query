/// Construct a `serde_json::Value` from a JSON literal
/// representing document.
///
/// This is equivalent to serde_json::json but also adds
/// _id field with uuid
///
/// ```
/// use memquery::{doc, errors::Error, memdb::MemDb, query};
///
/// let memdb = MemDb::new();
/// let coll = memdb.collection("TestCollection").await?;
///
/// coll.insert(doc!({ "name": "Tom", "age": 25 })).await?;
/// ```
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

/// Construct a `serde_json::Value` from a JSON literal
/// representing query spec.
///
///
/// ```
/// use memquery::{doc, errors::Error, memdb::MemDb, query};
///
/// let memdb = MemDb::new();
/// let coll = memdb.collection("TestCollection").await?;
///
/// coll.insert(doc!({ "name": "Tom", "age": 25 })).await?;
/// let docs = coll.find(query!({"name": "Tom", "age": 25})).await?;
/// ```
#[macro_export]
macro_rules! query {
  ($($json:tt)+) => {
    serde_json::json!($($json)+)
  };
}

/// Construct a `serde_json::Value` from a JSON literal
/// representing update value for find_and_update API.
///
///
/// ```
/// use memquery::{doc, errors::Error, memdb::MemDb, query};
///
/// let memdb = MemDb::new();
/// let coll = memdb.collection("TestCollection").await?;
///
/// coll.insert(doc!({ "name": "Tom", "age": 25 })).await?;
/// let docs_updated = coll
///  .find_and_update(
///    query!({ "name": "Tom" }),
///    update!({ "$set": { "name": "Roy", "age": 21, "email": "test@test.com" }}),
///  )
///  .await?;
/// ```
#[macro_export]
macro_rules! update {
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
