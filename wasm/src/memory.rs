use memquery::errors::Error;
use serde_json::{json, Value};

#[no_mangle]
pub fn alloc(len: usize) -> *mut u8 {
  let mut buf = Vec::with_capacity(len);
  let ptr = buf.as_mut_ptr();
  std::mem::forget(buf);
  ptr
}

#[no_mangle]
pub unsafe fn dealloc(ptr: *mut u8, size: usize) {
  // take ownership and deallocates
  let _ = Vec::from_raw_parts(ptr, size, size);
}

pub unsafe fn string_from_ptr(ptr: *mut u8, len: usize) -> String {
  let data = Vec::from_raw_parts(ptr, len, len);
  String::from_utf8_lossy(&data[..]).into_owned()
}

pub unsafe fn string_to_ptr(value: &str) -> *mut u8 {
  let mut b: Vec<u8> = value.as_bytes().iter().cloned().collect();
  b.push(0);
  let ptr = b.as_mut_ptr();
  std::mem::forget(b);
  ptr
}

pub unsafe fn json_from_ptr(ptr: *mut u8, len: usize) -> Result<Value, Error> {
  let jsonstr = string_from_ptr(ptr, len);
  let v = serde_json::from_str::<Value>(&jsonstr)?;
  Ok(v)
}

pub unsafe fn json_to_ptr(jsvalue: &Value) -> *mut u8 {
  let jsvalstr = jsvalue.to_string();
  string_to_ptr(&jsvalstr)
}

pub unsafe fn result_to_ptr(result: Result<&Value, Error>) -> *mut u8 {
  match result {
    Ok(v) => json_to_ptr(&json!({ "value": v })),
    Err(e) => json_to_ptr(&json!({ "error": json!(e.to_string()) })),
  }
}
