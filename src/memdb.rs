use super::{collection::Collection, errors::Error};
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(feature = "sync")]
use std::sync::Mutex;

#[cfg(not(feature = "sync"))]
use tokio::sync::Mutex;

pub struct MemDb {
  collections: Arc<Mutex<HashMap<String, Collection>>>,
}

impl MemDb {
  pub fn new() -> MemDb {
    MemDb {
      collections: Arc::new(Mutex::new(HashMap::new())),
    }
  }

  #[cfg(not(feature = "sync"))]
  pub async fn create_collection(&self, name: &str) {
    let new_collection = Collection::new();
    self
      .collections
      .lock()
      .await
      .insert(name.to_string(), new_collection);
  }

  #[cfg(not(feature = "sync"))]
  pub async fn collection(&self, collection_name: &str) -> Result<Collection, Error> {
    match self.collections.lock().await.get(collection_name) {
      Some(c) => Ok(c.clone()),
      None => Err(Error::MQCollectionNotFound),
    }
  }

  #[cfg(not(feature = "sync"))]
  pub async fn delete_collection(&self, name: &str) -> Result<Collection, Error> {
    self
      .collections
      .lock()
      .await
      .remove(name)
      .ok_or(Error::MQCollectionNotFound)
  }

  #[cfg(feature = "sync")]
  pub fn create_collection(&self, name: &str) {
    let new_collection = Collection::new();
    self
      .collections
      .lock()
      .unwrap()
      .insert(name.to_string(), new_collection);
  }

  #[cfg(feature = "sync")]
  pub fn collection(&self, collection_name: &str) -> Result<Collection, Error> {
    match self.collections.lock().unwrap().get(collection_name) {
      Some(c) => Ok(c.clone()),
      None => Err(Error::MQCollectionNotFound),
    }
  }

  #[cfg(feature = "sync")]
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
  use crate::{errors::Error, memdb::MemDb};

  #[cfg(not(feature = "sync"))]
  #[tokio::test]
  async fn test_create_collection() -> Result<(), Error> {
    let memdb = MemDb::new();
    memdb.create_collection("TestCollection").await;
    let _ = memdb.collection("TestCollection").await?;
    Ok(())
  }

  #[cfg(not(feature = "sync"))]
  #[tokio::test]
  async fn test_delete_collection() -> Result<(), Error> {
    let memdb = MemDb::new();
    memdb.create_collection("TestCollection").await;
    let _ = memdb.collection("TestCollection").await?;
    memdb.delete_collection("TestCollection").await?;
    if let Ok(_) = memdb.delete_collection("TestCollection").await {
      assert_eq!("should not find collection", "found collection");
    }

    Ok(())
  }

  #[cfg(feature = "sync")]
  #[test]
  fn test_create_collection() -> Result<(), Error> {
    let memdb = MemDb::new();
    memdb.create_collection("TestCollection");
    let _ = memdb.collection("TestCollection")?;
    Ok(())
  }

  #[cfg(feature = "sync")]
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
