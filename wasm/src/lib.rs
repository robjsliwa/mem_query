mod memory;

use lazy_static::lazy_static;
pub use memory::{
  alloc, dealloc, json_from_ptr, json_to_ptr, result_to_ptr, string_from_ptr, string_to_ptr,
};
use memquery::{doc, memdb::MemDb};
use serde_json::json;

lazy_static! {
  static ref MEMDB: MemDb = MemDb::new();
}

#[no_mangle]
pub fn create_collection(ptr: *mut u8, len: usize) {
  let name = unsafe { string_from_ptr(ptr, len) };
  MEMDB.create_collection(&name);
}

#[no_mangle]
pub fn collection(ptr: *mut u8, len: usize) -> u8 {
  let coll_name = unsafe { string_from_ptr(ptr, len) };
  match MEMDB.collection(&coll_name) {
    Ok(_) => 1,
    Err(_) => 0,
  }
}

#[no_mangle]
pub fn delete_collection(ptr: *mut u8, len: usize) -> u8 {
  let coll_name = unsafe { string_from_ptr(ptr, len) };
  match MEMDB.delete_collection(&coll_name) {
    Ok(_) => 1,
    Err(_) => 0,
  }
}

#[no_mangle]
pub fn insert(
  coll_name_ptr: *mut u8,
  coll_len: usize,
  doc_ptr: *mut u8,
  doc_len: usize,
) -> *mut u8 {
  let coll_name = unsafe { string_from_ptr(coll_name_ptr, coll_len) };
  let doc_res = unsafe { json_from_ptr(doc_ptr, doc_len) };

  let doc = match doc_res {
    Ok(d) => d,
    Err(e) => return unsafe { result_to_ptr(Err(e)) },
  };

  let coll = match MEMDB.collection(&coll_name) {
    Ok(c) => c,
    Err(e) => return unsafe { result_to_ptr(Err(e)) },
  };

  match coll.insert(doc.clone()) {
    Ok(_) => unsafe { result_to_ptr(Ok(&json!({}))) },
    Err(e) => unsafe { result_to_ptr(Err(e)) },
  }
}

#[no_mangle]
pub fn find(
  coll_name_ptr: *mut u8,
  coll_len: usize,
  query_ptr: *mut u8,
  query_len: usize,
) -> *mut u8 {
  let coll_name = unsafe { string_from_ptr(coll_name_ptr, coll_len) };
  let query_res = unsafe { json_from_ptr(query_ptr, query_len) };

  let query = match query_res {
    Ok(q) => q,
    Err(e) => return unsafe { result_to_ptr(Err(e)) },
  };

  let coll = match MEMDB.collection(&coll_name) {
    Ok(c) => c,
    Err(e) => return unsafe { result_to_ptr(Err(e)) },
  };

  match coll.find(query.clone()) {
    Ok(docs) => unsafe { result_to_ptr(Ok(&json!(docs))) },
    Err(e) => unsafe { result_to_ptr(Err(e)) },
  }
}

// pub type JsonDocument = Box<[JsValue]>;

// #[wasm_bindgen]
// pub struct MemoryDB {
//   mem_db: MemDb,
// }

// #[wasm_bindgen]
// impl MemoryDB {
//   #[wasm_bindgen(constructor)]
//   pub fn new() -> MemoryDB {
//     MemoryDB {
//       mem_db: MemDb::new(),
//     }
//   }

//   pub fn create_collection(&self, name: &str) {
//     self.mem_db.create_collection(name);
//   }

//   pub fn collection(&self, collection_name: &str) -> Result<JsValue, JsValue> {
//     match self.mem_db.collection(collection_name) {
//       Ok(v) => Ok(JsValue::from_str(
//         &json!(v.data.lock().unwrap()[..]).to_string(),
//       )),
//       Err(e) => Err(JsValue::from_str(&e.to_string())),
//     }
//   }

//   pub fn delete_collection(&self, name: &str) -> Result<JsValue, JsValue> {
//     match self.mem_db.delete_collection(name) {
//       Ok(v) => Ok(JsValue::from_str(
//         &json!(v.data.lock().unwrap()[..]).to_string(),
//       )),
//       Err(e) => Err(JsValue::from_str(&e.to_string())),
//     }
//   }
// }

#[no_mangle]
pub fn mytest(input: u8) -> u8 {
  input + 2
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
