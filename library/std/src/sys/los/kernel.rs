pub mod memory {
    extern "C" {
        pub fn allocate_memory(size: usize, alignment: usize) -> *mut u8;
        pub fn free_memory(ptr: *mut u8);
    }
}
