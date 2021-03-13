#[cfg(feature = "sync")]
use memquery::{doc, errors::Error, memdb::MemDb, query, update};

#[test]
#[cfg(feature = "sync")]
fn simple_update() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  let coll = memdb.collection("TestCollection")?;
  coll.insert(doc!({ "name": "Rob", "age": 25 }))?;
  coll.insert(doc!({ "name": "Bob", "age": 20 }))?;
  coll.insert(doc!({ "name": "Tom", "age": 30 }))?;

  let docs_updated = coll.find_and_update(
    query!({"name": "Bob"}),
    update!({"nickname": "Bobcat", "voice": "meow"}),
  )?;

  assert_eq!(docs_updated, 1);

  let docs = coll.find(query!({"nickname": "Bobcat"}))?;
  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["voice"], "meow");
  Ok(())
}

#[test]
#[cfg(feature = "sync")]
fn set_op_update() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  let coll = memdb.collection("TestCollection")?;
  coll.insert(doc!({ "name": "Rob", "age": 25 }))?;
  coll.insert(doc!({ "name": "Bob", "age": 20 }))?;
  coll.insert(doc!({ "name": "Tom", "age": 30 }))?;

  let docs_updated = coll.find_and_update(
    query!({"name": "Bob"}),
    update!({"$set": { "name": "Roy", "age": 21, "email": "test@test.com"}}),
  )?;

  assert_eq!(docs_updated, 1);

  let docs = coll.find(query!({"name": "Roy"}))?;
  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["age"], 21);
  assert_eq!(docs[0]["email"], "test@test.com");
  Ok(())
}

#[test]
#[cfg(feature = "sync")]
fn unset_op_update() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  let coll = memdb.collection("TestCollection")?;
  coll.insert(doc!({ "name": "Rob", "age": 25 }))?;
  coll.insert(doc!({ "name": "Bob", "age": 20 }))?;
  coll.insert(doc!({ "name": "Tom", "age": 30 }))?;

  let docs_updated = coll.find_and_update(
    query!({ "name": "Bob" }),
    update!({ "$set": { "name": "Roy", "age": 21, "email": "test@test.com" }}),
  )?;

  assert_eq!(docs_updated, 1);

  let docs = coll.find(query!({"name": "Roy"}))?;
  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["age"], 21);
  assert_eq!(docs[0]["email"], "test@test.com");

  let docs_updated2 = coll.find_and_update(
    query!({ "name": "Roy" }),
    update!({ "$unset": { "email": "" }}),
  )?;

  let docs = coll.find(query!({"name": "Roy"}))?;
  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["age"], 21);
  assert_eq!(docs[0]["email"], serde_json::Value::Null);

  Ok(())
}
