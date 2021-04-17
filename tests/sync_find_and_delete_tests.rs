#[cfg(feature = "sync")]
use memquery::{doc, errors::Error, memdb::MemDb, query};

#[test]
#[cfg(feature = "sync")]
fn simple_delete() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  let coll = memdb.collection("TestCollection")?;
  coll.insert(doc!({ "name": "Rob", "age": 25 }))?;
  coll.insert(doc!({ "name": "Bob", "age": 20 }))?;
  coll.insert(doc!({ "name": "Tom", "age": 30 }))?;

  let docs = coll.find_and_delete(query!({"name": "Bob"}))?;
  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["age"], 20);

  let docs_remaining = coll.find(query!({}))?;
  assert_eq!(docs_remaining.len(), 2);
  Ok(())
}

#[test]
#[cfg(feature = "sync")]
fn delete_all_docs() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  let coll = memdb.collection("TestCollection")?;
  coll.insert(doc!({ "name": "Rob", "age": 25 }))?;
  coll.insert(doc!({ "name": "Bob", "age": 20 }))?;
  coll.insert(doc!({ "name": "Tom", "age": 30 }))?;

  let docs = coll.find_and_delete(query!({}))?;
  assert_eq!(docs.len(), 3);

  let docs_remaining = coll.find(query!({}))?;
  assert_eq!(docs_remaining.len(), 0);

  Ok(())
}
