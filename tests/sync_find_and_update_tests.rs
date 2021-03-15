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

#[test]
#[cfg(feature = "sync")]
fn set_op_on_embedded_doc_update() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  let coll = memdb.collection("TestCollection")?;
  coll.insert(doc!({ "name": "Rob", "age": 25, "profile": { "email": "rob@test.com" } }))?;
  coll.insert(doc!({ "name": "Bob", "age": 20, "profile": { "email": "bob@test.com" }  }))?;
  coll.insert(doc!({ "name": "Tom", "age": 30, "profile": { "email": "tom@test.com" }  }))?;

  let docs_updated = coll.find_and_update(
    query!({"name": "Bob"}),
    update!({"$set": { "profile.email": "tom@test.com"}}),
  )?;

  assert_eq!(docs_updated, 1);

  let docs = coll.find(query!({"name": "Bob"}))?;
  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["age"], 20);
  assert_eq!(docs[0]["profile"]["email"], "tom@test.com");
  Ok(())
}

#[test]
#[cfg(feature = "sync")]
fn unset_op_on_embedded_doc_update() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  let coll = memdb.collection("TestCollection")?;
  coll.insert(doc!({ "name": "Rob", "age": 25, "profile": { "email": "rob@test.com" } }))?;
  coll.insert(doc!({ "name": "Bob", "age": 20, "profile": { "email": "bob@test.com" }  }))?;
  coll.insert(doc!({ "name": "Tom", "age": 30, "profile": { "email": "tom@test.com" }  }))?;

  let docs_updated = coll.find_and_update(
    query!({"name": "Bob"}),
    update!({"$set": { "profile.email": "tom@test.com"}}),
  )?;

  assert_eq!(docs_updated, 1);

  let docs = coll.find(query!({"name": "Bob"}))?;
  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["age"], 20);
  assert_eq!(docs[0]["profile"]["email"], "tom@test.com");

  let docs_updated = coll.find_and_update(
    query!({"name": "Bob"}),
    update!({"$unset": { "profile.email": "tom@test.com"}}),
  )?;

  assert_eq!(docs_updated, 1);

  let docs = coll.find(query!({"name": "Bob"}))?;
  assert_eq!(docs[0]["profile"]["email"], serde_json::Value::Null);

  Ok(())
}

#[test]
#[cfg(feature = "sync")]
fn inc_positive_op_update() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  let coll = memdb.collection("TestCollection")?;
  coll.insert(doc!({ "name": "Rob", "age": 25 }))?;
  coll.insert(doc!({ "name": "Bob", "age": 20 }))?;
  coll.insert(doc!({ "name": "Tom", "age": 30 }))?;

  let docs_updated =
    coll.find_and_update(query!({"name": "Bob"}), update!({"$inc": { "age": 5}}))?;

  assert_eq!(docs_updated, 1);

  let docs = coll.find(query!({"name": "Bob"}))?;
  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["age"], 25.0);
  Ok(())
}

#[test]
#[cfg(feature = "sync")]
fn inc_negative_op_update() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  let coll = memdb.collection("TestCollection")?;
  coll.insert(doc!({ "name": "Rob", "age": 25 }))?;
  coll.insert(doc!({ "name": "Bob", "age": 20 }))?;
  coll.insert(doc!({ "name": "Tom", "age": 30 }))?;

  let docs_updated =
    coll.find_and_update(query!({"name": "Bob"}), update!({"$inc": { "age": -5 }}))?;

  assert_eq!(docs_updated, 1);

  let docs = coll.find(query!({"name": "Bob"}))?;
  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["age"], 15.0);
  Ok(())
}

#[test]
#[cfg(feature = "sync")]
fn mul_positive_op_update() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  let coll = memdb.collection("TestCollection")?;
  coll.insert(doc!({ "name": "Rob", "age": 25 }))?;
  coll.insert(doc!({ "name": "Bob", "age": 20 }))?;
  coll.insert(doc!({ "name": "Tom", "age": 30 }))?;

  let docs_updated =
    coll.find_and_update(query!({"name": "Bob"}), update!({"$mul": { "age": 5}}))?;

  assert_eq!(docs_updated, 1);

  let docs = coll.find(query!({"name": "Bob"}))?;
  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["age"], 100.0);
  Ok(())
}

#[test]
#[cfg(feature = "sync")]
fn mul_negative_op_update() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  let coll = memdb.collection("TestCollection")?;
  coll.insert(doc!({ "name": "Rob", "age": 25 }))?;
  coll.insert(doc!({ "name": "Bob", "age": 20 }))?;
  coll.insert(doc!({ "name": "Tom", "age": 30 }))?;

  let docs_updated =
    coll.find_and_update(query!({"name": "Bob"}), update!({"$mul": { "age": -5 }}))?;

  assert_eq!(docs_updated, 1);

  let docs = coll.find(query!({"name": "Bob"}))?;
  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["age"], -100.0);
  Ok(())
}
