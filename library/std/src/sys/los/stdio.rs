use crate::io;

#[repr(C)]
struct FILE {
    class: usize,
    descriptor: isize,
    buffer_type: i32,
    buffer: *const u8,
    buffer_capacity: i32,
    buffer_length: i32,
    buffer_offset: i32,
    buffer_start: isize,
    unbuffered_buffer: i32,
    flags: i32,
    ungetc: *const u8,
}

pub struct Stdin;
pub struct Stdout;
pub struct Stderr;

pub const STDIN_BUF_SIZE: usize = 0;

extern "C" {
    static mut __stdin: FILE;
    static mut __stdout: FILE;
    static mut __stderr: FILE;

    fn fwrite(ptr: *const u8, size: usize, nmemb: usize, stream: *mut FILE) -> usize;
    fn fread(ptr: *mut u8, size: usize, nmemb: usize, stream: *mut FILE) -> usize;
    fn fflush(stream: *mut FILE);
}

impl Stdin {
    pub const fn new() -> Self {
        Stdin
    }
}

impl io::Read for Stdin {
    fn read(&mut self, data: &mut [u8]) -> io::Result<usize> {
        Ok(unsafe { fread(data.as_mut_ptr(), data.len(), 1, &mut __stdin) })
    }
}

unsafe impl Send for Stdin {}
unsafe impl Sync for Stdin {}

impl Stdout {
    pub const fn new() -> Self {
        Stdout
    }
}

impl io::Write for Stdout {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        Ok(unsafe { fwrite(data.as_ptr(), data.len(), 1, &mut __stdout) })
    }

    fn flush(&mut self) -> io::Result<()> {
        unsafe { fflush(&mut __stdout) };
        Ok(())
    }
}

unsafe impl Send for Stdout {}
unsafe impl Sync for Stdout {}

impl Stderr {
    pub const fn new() -> Self {
        Stderr
    }
}

impl io::Write for Stderr {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        Ok(unsafe { fwrite(data.as_ptr(), data.len(), 1, &mut __stderr) })
    }

    fn flush(&mut self) -> io::Result<()> {
        unsafe { fflush(&mut __stderr) };
        Ok(())
    }
}

unsafe impl Send for Stderr {}
unsafe impl Sync for Stderr {}

pub fn is_ebadf(_err: &io::Error) -> bool {
    true
}

pub fn panic_output() -> Option<impl io::Write> {
    Some(Stderr::new())
}
