// Should be using c_char and other ffi type definitions.
// Unsafe blocks don't enforce matching binding types of ptr borrow logic (only reference logic).
pub mod host_interface {

    use faasm_sys::{println, *};
    const KEY: *mut i8 = r"blah".as_ptr() as *mut i8;

    pub fn read_state() {
        let expected = [0, 1, 2];
        let value: &mut [u8] = &mut [8, 8, 8];

        unsafe {
            __faasm_read_state(KEY, value.as_ptr() as *mut u8, value.len() as i32);
        };

        println!("{:?}, {:?}", expected, value);
        assert_eq!(expected, value); // When this panics, it calls the wrong println. no_std?
    }


    pub fn write_state() {
        let value:&[u8] = &[0, 1, 2];

        unsafe { __faasm_write_state(KEY, value.as_ptr() as *mut u8, value.len() as i32); }
    }

}

// Don't compile OMP code natively because we don't want to support it
// Should be behind feature flag too
#[cfg(target_arch = "wasm32")]
pub mod omp {
    use faasm_sys::*;
    pub fn get_num_threads() -> i32 {
        unsafe { omp_get_num_threads() }
    }
}
