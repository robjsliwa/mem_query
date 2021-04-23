//! Collection stores documents as JSON objects.
//!
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

/// Stores JSON documents.
#[derive(Clone)]
pub struct Collection {
  pub data: DocumentCollection,
}

impl Collection {
  /// Make a new collection.
  pub fn new() -> Collection {
    Collection {
      data: Arc::new(Mutex::new(Vec::new())),
    }
  }

  /// Insert new document.
  ///
  /// ```
  /// coll.insert(doc!({ "name": "Tom", "age": 25 }))?;
  /// ```
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

  /// Insert new document.
  ///
  /// ```
  /// coll.insert(doc!({ "name": "Tom", "age": 25 })).await?;
  /// ```
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

  /// Return documents that match specified criteria.
  ///
  /// ```
  /// let docs = coll.find(query!({"name": "Tom", "age": 25}))?;
  /// ```
  #[cfg(feature = "sync")]
  pub fn find(&self, query: Value) -> Result<Documents, Error> {
    if !query.is_object() {
      return Err(Error::MQError(String::from("Query must be a JSON object.")));
    }

    Engine::with_collection(self.data.clone()).find(&query)
  }

  /// Return documents that match specified criteria.
  ///
  /// ```
  /// let docs = coll.find(query!({"name": "Tom", "age": 25})).await?;
  /// ```
  #[cfg(not(feature = "sync"))]
  pub async fn find(&self, query: Value) -> Result<Documents, Error> {
    if !query.is_object() {
      return Err(Error::MQError(String::from("Query must be a JSON object.")));
    }

    Engine::with_collection(self.data.clone())
      .find(&query)
      .await
  }

  /// Updates documents that match search criteria.
  ///
  /// ```
  /// let docs_updated = coll
  ///   .find_and_update(
  ///   query!({"name": "Bob"}),
  ///   update!({"nickname": "Bobcat", "voice": "meow"}),
  /// )?;
  /// ```
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

  /// Updates documents that match search criteria.
  ///
  /// ```
  /// let docs_updated = coll
  ///   .find_and_update(
  ///   query!({"name": "Bob"}),
  ///   update!({"nickname": "Bobcat", "voice": "meow"}),
  /// )
  /// .await?;
  /// ```
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

  /// Delete documents that match search criteria.
  ///
  /// ```
  /// let docs = coll.find_and_delete(query!({"name": "Bob"}))?;
  /// ```
  #[cfg(feature = "sync")]
  pub fn find_and_delete(&self, query: Value) -> Result<Documents, Error> {
    if !query.is_object() {
      return Err(Error::MQError(String::from("Query must be a JSON object.")));
    }

    Engine::with_collection(self.data.clone()).find_and_delete(&query)
  }

  /// Delete documents that match search criteria.
  ///
  /// ```
  /// let docs = coll.find_and_delete(query!({"name": "Bob"})).await?;
  /// ```
  #[cfg(not(feature = "sync"))]
  pub async fn find_and_delete(&self, query: Value) -> Result<Documents, Error> {
    if !query.is_object() {
      return Err(Error::MQError(String::from("Query must be a JSON object.")));
    }

    Engine::with_collection(self.data.clone())
      .find_and_delete(&query)
      .await
  }
}
