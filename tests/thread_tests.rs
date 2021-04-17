#[cfg(feature = "sync")]
use memquery::{doc, errors::Error, memdb::MemDb, query, update};
#[cfg(feature = "sync")]
use std::thread;

#[test]
#[cfg(feature = "sync")]
fn test_simple_query_in_thread() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  let coll = memdb.collection("TestCollection")?;

  let coll1 = coll.clone();
  let h1 = thread::spawn(move || {
    let _ = coll1.insert(doc!({ "name": "Rob", "age": 25 }));
  });

  let coll2 = coll.clone();
  let h2 = thread::spawn(move || {
    let _ = coll2.insert(doc!({ "name": "Bob", "age": 20 }));
  });

  let coll3 = coll.clone();
  let h3 = thread::spawn(move || {
    let _ = coll3.insert(doc!({ "name": "Tom", "age": 30 }));
  });

  let _ = h1.join();
  let _ = h2.join();
  let _ = h3.join();

  let docs = coll.find(query!({"name": "Bob"}))?;

  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["name"], "Bob");
  Ok(())
}
#[test]
#[cfg(feature = "sync")]
fn test_find_and_update_in_threads() -> Result<(), Error> {
  let memdb = MemDb::new();
  memdb.create_collection("TestCollection");
  let coll = memdb.collection("TestCollection")?;

  let coll_t1 = coll.clone();
  let h1 = thread::spawn(move || {
    let _ = coll_t1.insert(doc!({ "name": "Rob", "age": 25 }));
  });

  let coll_t2 = coll.clone();
  let h2 = thread::spawn(move || {
    let _ = coll_t2.insert(doc!({ "name": "Bob", "age": 20 }));
  });

  let coll_t3 = coll.clone();
  let h3 = thread::spawn(move || {
    let _ = coll_t3.insert(doc!({ "name": "Tom", "age": 30 }));
  });

  let _ = h1.join();
  let _ = h2.join();
  let _ = h3.join();

  let coll_t4 = coll.clone();
  let h4 = thread::spawn(move || {
    let _ = coll_t4.find_and_update(
      query!({"name": "Bob"}),
      update!({"nickname": "Bobcat", "voice": "meow"}),
    );
  });

  let _ = h4.join();

  let docs = coll.find(query!({"nickname": "Bobcat"}))?;

  assert_eq!(docs.len(), 1);
  assert_eq!(docs[0]["voice"], "meow");
  Ok(())
}
