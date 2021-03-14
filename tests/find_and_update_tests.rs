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
async fn set_op_invalid_value_update() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
  coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
  coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;

  if let Ok(_) = coll
    .find_and_update(
      query!({"name": "Bob"}),
      update!({"$set": { "$name": "Roy", "age": 21, "email": "test@test.com"}}),
    )
    .await
  {
    assert_eq!("should get error", "no error");
  }

  Ok(())
}

#[tokio::test]
#[cfg(not(feature = "sync"))]
async fn set_op_invalid_value_embedded_update() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
  coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
  coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;

  if let Ok(_) = coll
    .find_and_update(
      query!({"name": "Bob"}),
      update!({"$set": { "name": "Roy", "age.$set": 21, "email": "test@test.com"}}),
    )
    .await
  {
    assert_eq!("should get error", "no error");
  }

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

  assert_eq!(docs_updated2, 1);

  let docs = coll.find(query!({"name": "Roy"})).await?;
  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["age"], 21);
  assert_eq!(docs[0]["email"], serde_json::Value::Null);

  Ok(())
}

#[tokio::test]
#[cfg(not(feature = "sync"))]
async fn set_op_on_embedded_doc_update() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll
    .insert(doc!({ "name": "Rob", "age": 25, "profile": { "email": "rob@test.com" } }))
    .await?;
  coll
    .insert(doc!({ "name": "Bob", "age": 20, "profile": { "email": "bob@test.com" }  }))
    .await?;
  coll
    .insert(doc!({ "name": "Tom", "age": 30, "profile": { "email": "tom@test.com" }  }))
    .await?;

  let docs_updated = coll
    .find_and_update(
      query!({"name": "Bob"}),
      update!({"$set": { "profile.email": "tom@test.com"}}),
    )
    .await?;

  assert_eq!(docs_updated, 1);

  let docs = coll.find(query!({"name": "Bob"})).await?;
  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["age"], 20);
  assert_eq!(docs[0]["profile"]["email"], "tom@test.com");
  Ok(())
}

#[tokio::test]
#[cfg(not(feature = "sync"))]
async fn unset_op_on_embedded_doc_update() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll
    .insert(doc!({ "name": "Rob", "age": 25, "profile": { "email": "rob@test.com" } }))
    .await?;
  coll
    .insert(doc!({ "name": "Bob", "age": 20, "profile": { "email": "bob@test.com" }  }))
    .await?;
  coll
    .insert(doc!({ "name": "Tom", "age": 30, "profile": { "email": "tom@test.com" }  }))
    .await?;

  let docs_updated = coll
    .find_and_update(
      query!({"name": "Bob"}),
      update!({"$set": { "profile.email": "tom@test.com"}}),
    )
    .await?;

  assert_eq!(docs_updated, 1);

  let docs = coll.find(query!({"name": "Bob"})).await?;
  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["age"], 20);
  assert_eq!(docs[0]["profile"]["email"], "tom@test.com");

  let docs_updated = coll
    .find_and_update(
      query!({"name": "Bob"}),
      update!({"$unset": { "profile.email": "tom@test.com"}}),
    )
    .await?;

  assert_eq!(docs_updated, 1);

  let docs = coll.find(query!({"name": "Bob"})).await?;
  assert_eq!(docs[0]["profile"]["email"], serde_json::Value::Null);

  Ok(())
}

#[tokio::test]
#[cfg(not(feature = "sync"))]
async fn inc_positive_op_update() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
  coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
  coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;

  let docs_updated = coll
    .find_and_update(query!({"name": "Bob"}), update!({"$inc": { "age": 5}}))
    .await?;

  assert_eq!(docs_updated, 1);

  let docs = coll.find(query!({"name": "Bob"})).await?;
  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["age"], 25.0);
  Ok(())
}

#[tokio::test]
#[cfg(not(feature = "sync"))]
async fn inc_negative_op_update() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
  coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
  coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;

  let docs_updated = coll
    .find_and_update(query!({"name": "Bob"}), update!({"$inc": { "age": -5 }}))
    .await?;

  assert_eq!(docs_updated, 1);

  let docs = coll.find(query!({"name": "Bob"})).await?;
  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["age"], 15.0);
  Ok(())
}

#[tokio::test]
#[cfg(not(feature = "sync"))]
async fn mul_positive_op_update() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
  coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
  coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;

  let docs_updated = coll
    .find_and_update(query!({"name": "Bob"}), update!({"$mul": { "age": 5}}))
    .await?;

  assert_eq!(docs_updated, 1);

  let docs = coll.find(query!({"name": "Bob"})).await?;
  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["age"], 100.0);
  Ok(())
}

#[tokio::test]
#[cfg(not(feature = "sync"))]
async fn mul_negative_op_update() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;
  coll.insert(doc!({ "name": "Rob", "age": 25 })).await?;
  coll.insert(doc!({ "name": "Bob", "age": 20 })).await?;
  coll.insert(doc!({ "name": "Tom", "age": 30 })).await?;

  let docs_updated = coll
    .find_and_update(query!({"name": "Bob"}), update!({"$mul": { "age": -5 }}))
    .await?;

  assert_eq!(docs_updated, 1);

  let docs = coll.find(query!({"name": "Bob"})).await?;
  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["age"], -100.0);
  Ok(())
}
