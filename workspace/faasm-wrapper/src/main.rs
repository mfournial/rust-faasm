// Printf
//extern "C" {
//    fn printf(fmt: *const i8, ...) -> i32;
//}
// *** 
//    unsafe {
//        printf("hello world\n\0".as_ptr() as *const i8);
//    }
// ***
// use std::io::{self, Read, Write};
// std::io::stdio...

// use log::{Level, info};
// console_log::init_with_level(Level::Debug).unwrap();
// info!("It works!");

extern crate faasm_wrapper;

use faasm_wrapper::host_interface::{read_state, write_state};

fn main() {
    write_state();
    read_state();
}

/*
use faasm_wrapper::omp;

fn main() {
    assert_eq!(omp::get_num_threads(), 1);
}
*/
