use super::{
  engine::{DocumentCollection, Documents, Engine},
  errors::Error,
};
use serde_json::Value;
use std::sync::Arc;

#[cfg(feature = "sync")]
use std::sync::Mutex;

#[cfg(not(feature = "sync"))]
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Collection {
  pub data: DocumentCollection,
}

impl Collection {
  pub fn new() -> Collection {
    Collection {
      data: Arc::new(Mutex::new(Vec::new())),
    }
  }

  #[cfg(feature = "sync")]
  pub fn insert(&self, document: Value) -> Result<(), Error> {
    if !document.is_object() {
      return Err(Error::MQError(String::from(
        "Document must be a JSON object.",
      )));
    }

    self.data.lock().unwrap().push(document);

    Ok(())
  }

  #[cfg(not(feature = "sync"))]
  pub async fn insert(&self, document: Value) -> Result<(), Error> {
    if !document.is_object() {
      return Err(Error::MQError(String::from(
        "Document must be a JSON object.",
      )));
    }

    self.data.lock().await.push(document);

    Ok(())
  }

  #[cfg(feature = "sync")]
  pub fn find(&self, query: Value) -> Result<Documents, Error> {
    if !query.is_object() {
      return Err(Error::MQError(String::from("Query must be a JSON object.")));
    }

    Engine::with_collection(self.data.clone()).find(&query)
  }

  #[cfg(not(feature = "sync"))]
  pub async fn find(&self, query: Value) -> Result<Documents, Error> {
    if !query.is_object() {
      return Err(Error::MQError(String::from("Query must be a JSON object.")));
    }

    Engine::with_collection(self.data.clone())
      .find(&query)
      .await
  }

  #[cfg(feature = "sync")]
  pub fn find_and_update(&self, query: Value, update: Value) -> Result<u64, Error> {
    if !query.is_object() {
      return Err(Error::MQError(String::from("Query must be a JSON object.")));
    }

    if !update.is_object() {
      return Err(Error::MQError(String::from(
        "Update must be a JSON object.",
      )));
    }

    Engine::with_collection(self.data.clone()).find_and_update(&query, &update)
  }

  #[cfg(not(feature = "sync"))]
  pub async fn find_and_update(&self, query: Value, update: Value) -> Result<u64, Error> {
    if !query.is_object() {
      return Err(Error::MQError(String::from("Query must be a JSON object.")));
    }

    if !update.is_object() {
      return Err(Error::MQError(String::from(
        "Update must be a JSON object.",
      )));
    }

    Engine::with_collection(self.data.clone())
      .find_and_update(&query, &update)
      .await
  }
}
