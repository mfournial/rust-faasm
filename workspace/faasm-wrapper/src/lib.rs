// Re-export convinience println in wasm prelude
pub mod prelude {
    #[cfg(target_arch = "wasm32")]
    pub use faasm_sys::{__println, println};
}

pub mod host_interface {
    use faasm_sys::*;

    use std::ffi::CStr;
    use std::os::raw::{c_long, c_uchar};

    pub fn read_state(key: &CStr, state_size: usize) -> Vec<c_uchar> {
        let key_ptr = key.as_ptr();
        let mut vec: Vec<c_uchar> = Vec::with_capacity(state_size);

        unsafe {
            __faasm_read_state(key_ptr, vec.as_mut_ptr(), vec.capacity() as c_long);
            // Manually sets the size of the vector
            vec.set_len(state_size);
        };

        vec
    }

    pub fn write_state(key: &CStr, state: &[c_uchar]) {
        let key_ptr = key.as_ptr();
        unsafe {
            __faasm_write_state(key_ptr, state.as_ptr(), state.len() as c_long);
        }
    }
}
