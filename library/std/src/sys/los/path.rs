use crate::{ffi::OsStr, path::Prefix};

pub const MAIN_SEP_STR: &str = "/";
pub const MAIN_SEP: char = '/';

#[inline]
pub fn is_sep_byte(b: u8) -> bool {
    b == b'/' || b == b'\\'
}

#[inline]
pub fn is_verbatim_sep(b: u8) -> bool {
    b == b'/'
}

pub fn parse_prefix(_path: &OsStr) -> Option<Prefix<'_>> {
    None
}
