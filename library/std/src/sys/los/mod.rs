use crate::{io::ErrorKind, os::raw::c_char};

mod kernel;

pub mod alloc;
pub mod args;
#[path = "../unix/cmath.rs"]
pub mod cmath;
#[path = "../unsupported/condvar.rs"]
pub mod condvar;
pub mod env;
pub mod fs;
pub mod io;
pub mod memchr;
#[path = "../unsupported/mutex.rs"]
pub mod mutex;
pub mod net;
pub mod os;
#[path = "../unix/os_str.rs"]
pub mod os_str;
pub mod path;
pub mod pipe;
pub mod process;
#[path = "../unsupported/rwlock.rs"]
pub mod rwlock;
pub mod stdio;
pub mod thread;
pub mod thread_local_dtor;
#[path = "../unsupported/thread_local_key.rs"]
pub mod thread_local_key;
pub mod time;

extern "C" {
    fn abort() -> !;
}

pub fn abort_internal() -> ! {
    unsafe { abort() };
}

pub unsafe fn init(_argc: isize, _argv: *const *const u8) {}

pub unsafe fn cleanup() {}

pub fn decode_error_kind(_errno: i32) -> ErrorKind {
    ErrorKind::Uncategorized
}

pub unsafe fn strlen(start: *const c_char) -> usize {
    let mut str = start;

    while *str != 0 {
        str = str.offset(1);
    }

    (str as usize) - (start as usize)
}

pub fn hashmap_random_keys() -> (u64, u64) {
    (1, 2)
}

pub fn unsupported<T>() -> crate::io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> crate::io::Error {
    crate::io::Error::new_const(
        crate::io::ErrorKind::Unsupported,
        &"operation not supported on LOS yet",
    )
}
