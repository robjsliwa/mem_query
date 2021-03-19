use lazy_static::lazy_static;
use memquery::{collection::Collection, doc, errors::Error, memdb::MemDb, query};
use serde_json::json;

lazy_static! {
  static ref MEMDB: MemDb = MemDb::new();
}

#[no_mangle]
pub fn alloc(len: usize) -> *mut u8 {
  let mut buf = Vec::with_capacity(len);
  let ptr = buf.as_mut_ptr();
  std::mem::forget(buf);
  ptr
}

#[no_mangle]
pub unsafe fn dealloc(ptr: *mut u8, size: usize) {
  let data = Vec::from_raw_parts(ptr, size, size);

  std::mem::drop(data);
}

pub unsafe fn string_from_ptr(ptr: *mut u8, len: usize) -> String {
  let data = Vec::from_raw_parts(ptr, len, len);
  String::from_utf8_lossy(&data[..]).into_owned()
}

pub unsafe fn create_collection(ptr: *mut u8, len: usize) {
  let name = string_from_ptr(ptr, len);
  MEMDB.create_collection(&name);
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
