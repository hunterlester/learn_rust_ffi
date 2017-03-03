extern crate rand;
extern crate rustc_serialize;
extern crate libc;
use libc::{c_char};
use rand::{OsRng, Rng};
use std::ffi::CString;
use rustc_serialize::json::{ self };

#[no_mangle]
pub extern "C" fn generate() -> *mut c_char {

  let choices: [u8; 6] = [1, 2, 3, 4, 5, 6];

  let mut rand_num_vec: Vec<u8> = Vec::new();

  let mut rng = match OsRng::new() {
    Ok(t) => t,
    Err(e) => panic!("FAIL!, {}", e),
  };

  for _ in 0..6 {
    rand_num_vec.push(*rng.choose(&choices).unwrap());
  }

  let json_string = CString::new(json::encode(&rand_num_vec).unwrap()).unwrap();

  json_string.into_raw()

}

#[no_mangle]
pub extern "C" fn free_memory(pointer: *mut c_char) {
  unsafe {
    if pointer.is_null() { return }
    CString::from_raw(pointer)
  };
}
