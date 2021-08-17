use crate::{ffi::CStr, io, num::NonZeroUsize, sys::unsupported, time::Duration};

#[cfg_attr(test, allow(dead_code))]
pub mod guard {
    pub type Guard = !;
    pub unsafe fn current() -> Option<Guard> {
        None
    }
    pub unsafe fn init() -> Option<Guard> {
        None
    }
}

pub struct Thread;

pub const DEFAULT_MIN_STACK_SIZE: usize = 32 * 1024;

impl Thread {
    pub unsafe fn new(_stack: usize, _p: Box<dyn FnOnce()>) -> io::Result<Thread> {
        Ok(Thread)
    }

    pub fn yield_now() {}

    pub fn set_name(_name: &CStr) {}

    pub fn sleep(_dur: Duration) {}

    pub fn join(self) {}
}

pub fn available_concurrency() -> io::Result<NonZeroUsize> {
    unsupported()
}
