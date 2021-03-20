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
