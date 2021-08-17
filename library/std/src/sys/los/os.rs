use crate::{
    ffi::{OsStr, OsString},
    fmt, io,
    marker::PhantomData,
    path,
    path::PathBuf,
    sys::unsupported,
};

pub struct SplitPaths<'a>(!, PhantomData<&'a ()>);

pub struct Env;

pub fn env() -> Env {
    Env
}

pub fn getenv(_k: &OsStr) -> Option<OsString> {
    None
}

pub fn setenv(_k: &OsStr, _v: &OsStr) -> io::Result<()> {
    unsupported()
}

pub fn unsetenv(_k: &OsStr) -> io::Result<()> {
    unsupported()
}

pub fn getcwd() -> io::Result<PathBuf> {
    unsupported()
}

pub fn chdir(_: &path::Path) -> io::Result<()> {
    unsupported()
}

pub fn errno() -> i32 {
    0
}

pub fn error_string(_errno: i32) -> String {
    "error".to_string()
}

pub fn exit(_code: i32) -> ! {
    loop {}
}

pub fn getpid() -> u32 {
    0
}

pub fn current_exe() -> io::Result<PathBuf> {
    unsupported()
}

pub fn temp_dir() -> PathBuf {
    PathBuf::new()
}

pub fn home_dir() -> Option<PathBuf> {
    None
}

#[derive(Debug)]
pub struct JoinPathsError;

pub fn join_paths<I, T>(_paths: I) -> Result<OsString, JoinPathsError>
where
    I: Iterator<Item = T>,
    T: AsRef<OsStr>,
{
    Err(JoinPathsError)
}

pub fn split_paths(_unparsed: &OsStr) -> SplitPaths<'_> {
    panic!("unsupported")
}

impl fmt::Display for JoinPathsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "".fmt(f)
    }
}

impl crate::error::Error for JoinPathsError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        ""
    }
}

impl<'a> Iterator for SplitPaths<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl Iterator for Env {
    type Item = (OsString, OsString);
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
