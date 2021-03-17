use memquery::{collection::Collection, doc, errors::Error, memdb::MemDb, query};
use serde_json::json;
use wasm_bindgen::prelude::*;

pub type JsonDocument = Box<[JsValue]>;

#[wasm_bindgen]
pub struct MemoryDB {
  mem_db: MemDb,
}

#[wasm_bindgen]
impl MemoryDB {
  #[wasm_bindgen(constructor)]
  pub fn new() -> MemoryDB {
    MemoryDB {
      mem_db: MemDb::new(),
    }
  }

  pub fn create_collection(&self, name: &str) {
    self.mem_db.create_collection(name);
  }

  pub fn collection(&self, collection_name: &str) -> Result<JsValue, JsValue> {
    match self.mem_db.collection(collection_name) {
      Ok(v) => Ok(JsValue::from_str(
        &json!(v.data.lock().unwrap()[..]).to_string(),
      )),
      Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
  }

  pub fn delete_collection(&self, name: &str) -> Result<JsValue, JsValue> {
    match self.mem_db.delete_collection(name) {
      Ok(v) => Ok(JsValue::from_str(
        &json!(v.data.lock().unwrap()[..]).to_string(),
      )),
      Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
