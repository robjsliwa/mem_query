#[cfg(not(feature = "sync"))]
use memquery::{doc, errors::Error, memdb::MemDb, query, update};

#[tokio::test]
#[cfg(not(feature = "sync"))]
async fn simple_update() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
  coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
  coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;

  let docs_updated = coll
    .find_and_update(
      query!({"name": "Bob"}),
      update!({"nickname": "Bobcat", "voice": "meow"}),
    )
    .await?;

  assert_eq!(docs_updated, 1);

  let docs = coll.find(query!({"nickname": "Bobcat"})).await?;
  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["voice"], "meow");
  Ok(())
}

#[tokio::test]
#[cfg(not(feature = "sync"))]
async fn set_op_update() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
  coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
  coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;

  let docs_updated = coll
    .find_and_update(
      query!({"name": "Bob"}),
      update!({"$set": { "name": "Roy", "age": 21, "email": "test@test.com"}}),
    )
    .await?;

  assert_eq!(docs_updated, 1);

  let docs = coll.find(query!({"name": "Roy"})).await?;
  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["age"], 21);
  assert_eq!(docs[0]["email"], "test@test.com");
  Ok(())
}

#[tokio::test]
#[cfg(not(feature = "sync"))]
async fn unset_op_update() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
  coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
  coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;

  let docs_updated = coll
    .find_and_update(
      query!({ "name": "Bob" }),
      update!({ "$set": { "name": "Roy", "age": 21, "email": "test@test.com" }}),
    )
    .await?;

  assert_eq!(docs_updated, 1);

  let docs = coll.find(query!({"name": "Roy"})).await?;
  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["age"], 21);
  assert_eq!(docs[0]["email"], "test@test.com");

  let docs_updated2 = coll
    .find_and_update(
      query!({ "name": "Roy" }),
      update!({ "$unset": { "email": "" }}),
    )
    .await?;

  let docs = coll.find(query!({"name": "Roy"})).await?;
  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["age"], 21);
  assert_eq!(docs[0]["email"], serde_json::Value::Null);

  Ok(())
}
