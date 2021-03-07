use super::{
  engine::{Documents, Engine},
  errors::Error,
};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

type DocumentCollection = Arc<Mutex<Vec<Value>>>;

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

  pub async fn insert(&self, document: Value) -> Result<(), Error> {
    if !document.is_object() {
      return Err(Error::MQError(String::from(
        "Document must be a JSON object.",
      )));
    }

    self.data.lock().await.push(document);

    Ok(())
  }

  pub async fn find(&self, query: Value) -> Result<Documents, Error> {
    if !query.is_object() {
      return Err(Error::MQError(String::from("Query must be a JSON object.")));
    }

    Engine::with_collection(&*self.data.lock().await).find(&query)
  }
}
