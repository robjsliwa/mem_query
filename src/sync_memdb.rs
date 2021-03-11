use super::{errors::Error, sync_collection::Collection};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct MemDb {
  collections: Arc<Mutex<HashMap<String, Collection>>>,
}

impl MemDb {
  pub fn new() -> MemDb {
    MemDb {
      collections: Arc::new(Mutex::new(HashMap::new())),
    }
  }

  pub fn create_collection(&self, name: &str) {
    let new_collection = Collection::new();
    self
      .collections
      .lock()
      .unwrap()
      .insert(name.to_string(), new_collection);
  }

  pub fn collection(&self, collection_name: &str) -> Result<Collection, Error> {
    match self.collections.lock().unwrap().get(collection_name) {
      Some(c) => Ok(c.clone()),
      None => Err(Error::MQCollectionNotFound),
    }
  }

  pub fn delete_collection(&self, name: &str) -> Result<Collection, Error> {
    self
      .collections
      .lock()
      .unwrap()
      .remove(name)
      .ok_or(Error::MQCollectionNotFound)
  }
}

#[cfg(test)]
mod tests {
  use crate::{errors::Error, sync_memdb::MemDb};

  #[test]
  fn test_create_collection() -> Result<(), Error> {
    let memdb = MemDb::new();
    memdb.create_collection("TestCollection");
    let _ = memdb.collection("TestCollection")?;
    Ok(())
  }

  #[test]
  fn test_delete_collection() -> Result<(), Error> {
    let memdb = MemDb::new();
    memdb.create_collection("TestCollection");
    let _ = memdb.collection("TestCollection")?;
    memdb.delete_collection("TestCollection")?;
    if let Ok(_) = memdb.delete_collection("TestCollection") {
      assert_eq!("should not find collection", "found collection");
    }

    Ok(())
  }
}
