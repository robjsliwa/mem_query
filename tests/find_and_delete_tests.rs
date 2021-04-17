#[cfg(not(feature = "sync"))]
use memquery::{doc, errors::Error, memdb::MemDb, query};

#[tokio::test]
#[cfg(not(feature = "sync"))]
async fn simple_delete() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
  coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
  coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;

  let docs = coll.find_and_delete(query!({"name": "Bob"})).await?;
  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["age"], 20);

  let docs_remaining = coll.find(query!({})).await?;
  assert_eq!(docs_remaining.len(), 2);
  Ok(())
}

#[tokio::test]
#[cfg(not(feature = "sync"))]
async fn delete_all_docs() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
  coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
  coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;

  let docs = coll.find_and_delete(query!({})).await?;
  assert_eq!(docs.len(), 3);

  let docs_remaining = coll.find(query!({})).await?;
  assert_eq!(docs_remaining.len(), 0);

  Ok(())
}
