#[cfg(not(feature = "sync"))]
use memquery::{doc, errors::Error, memdb::MemDb, query, update};

#[tokio::test]
#[cfg(not(feature = "sync"))]
async fn test_simple_query_in_tasks() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;

  let coll_t1 = coll.clone();
  let h1 = tokio::spawn(async move {
    let _ = coll_t1.insert(doc!({ "name": "Rob", "age": 25 })).await;
  });

  let coll_t2 = coll.clone();
  let h2 = tokio::spawn(async move {
    let _ = coll_t2.insert(doc!({ "name": "Bob", "age": 20 })).await;
  });

  let coll_t3 = coll.clone();
  let h3 = tokio::spawn(async move {
    let _ = coll_t3.insert(doc!({ "name": "Tom", "age": 30 })).await;
  });

  h1.await.unwrap();
  h2.await.unwrap();
  h3.await.unwrap();

  let docs = coll.find(query!({"name": "Bob"})).await?;

  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["name"], "Bob");
  Ok(())
}

#[tokio::test]
#[cfg(not(feature = "sync"))]
async fn test_find_and_update_in_tasks() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection").await;
  let coll = memdb.collection("TestCollection").await?;

  let coll_t1 = coll.clone();
  let h1 = tokio::spawn(async move {
    let _ = coll_t1.insert(doc!({ "name": "Rob", "age": 25 })).await;
  });

  let coll_t2 = coll.clone();
  let h2 = tokio::spawn(async move {
    let _ = coll_t2.insert(doc!({ "name": "Bob", "age": 20 })).await;
  });

  let coll_t3 = coll.clone();
  let h3 = tokio::spawn(async move {
    let _ = coll_t3.insert(doc!({ "name": "Tom", "age": 30 })).await;
  });

  h1.await.unwrap();
  h2.await.unwrap();
  h3.await.unwrap();

  let coll_t4 = coll.clone();
  let h4 = tokio::spawn(async move {
    let docs_updated = coll_t4
      .find_and_update(
        query!({"name": "Bob"}),
        update!({"nickname": "Bobcat", "voice": "meow"}),
      )
      .await;

    match docs_updated {
      Ok(n) => n,
      Err(_) => 0,
    }
  });

  let docs_updated = h4.await.unwrap();
  assert_eq!(docs_updated, 1);

  let docs = coll.find(query!({"nickname": "Bobcat"})).await?;

  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["voice"], "meow");
  Ok(())
}
