use super::{collection::Collection, errors::Error};
use std::collections::HashMap;
use std::sync::Arc;
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

  pub async fn create_collection(&self, name: &str) {
    let new_collection = Collection::new();
    self
      .collections
      .lock()
      .await
      .insert(name.to_string(), new_collection);
  }

  pub async fn collection(&self, collection_name: &str) -> Result<Collection, Error> {
    match self.collections.lock().await.get(collection_name) {
      Some(c) => Ok(c.clone()),
      None => Err(Error::MQCollectionNotFound),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::{errors::Error, memdb::MemDb};

  #[tokio::test]
  async fn test_create_collection() -> Result<(), Error> {
    let memdb = MemDb::new();
    memdb.create_collection("TestCollection").await;
    let _ = memdb.collection("TestCollection").await?;
    Ok(())
  }
}
