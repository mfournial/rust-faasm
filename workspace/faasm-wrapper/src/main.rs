#[cfg(target_arch = "wasm32")]
// Explicitly import println! macro to shadow std's println!
use faasm_wrapper::prelude::println;

use faasm_wrapper::host_interface::{read_state, write_state};
use faasm_wrapper::prelude::*;

use std::ffi::CString;

fn main() {
    let key = CString::new("Hello, Faasm!").unwrap();
    let original = vec![0, 1, 2];
    write_state(&key, &original);
    let result = read_state(&key, original.len());

    println!("{:?}, {:?}", original, result);
    assert_eq!(original, result); // When this panics, it calls the wrong println. no_std?
}
