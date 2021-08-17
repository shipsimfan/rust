use crate::{
    ffi::OsString,
    io::{self, IoSlice, IoSliceMut, SeekFrom},
    path::{Path, PathBuf},
    sys::{time::SystemTime, unsupported},
};

pub use crate::sys_common::fs::{copy, try_exists};

#[derive(Debug)]
pub struct DirBuilder;

#[derive(Debug, Clone)]
pub struct OpenOptions;

#[derive(Debug)]
pub struct ReadDir;

#[derive(Clone)]
pub struct FileAttr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilePermissions;

pub struct DirEntry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileType;

#[derive(Debug)]
pub struct File;

impl DirBuilder {
    pub fn new() -> Self {
        DirBuilder
    }

    pub fn mkdir(&self, _p: &Path) -> io::Result<()> {
        unsupported()
    }
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        PathBuf::new()
    }

    pub fn file_name(&self) -> OsString {
        OsString::new()
    }

    pub fn metadata(&self) -> io::Result<FileAttr> {
        Ok(FileAttr)
    }

    pub fn file_type(&self) -> io::Result<FileType> {
        Ok(FileType)
    }
}

impl OpenOptions {
    pub fn new() -> Self {
        OpenOptions
    }

    pub fn read(&mut self, _read: bool) {}
    pub fn write(&mut self, _write: bool) {}
    pub fn append(&mut self, _append: bool) {}
    pub fn truncate(&mut self, _truncate: bool) {}
    pub fn create(&mut self, _create: bool) {}
    pub fn create_new(&mut self, _create_new: bool) {}
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl FileType {
    pub fn is_dir(&self) -> bool {
        false
    }

    pub fn is_file(&self) -> bool {
        false
    }

    pub fn is_symlink(&self) -> bool {
        false
    }
}

impl FilePermissions {
    pub fn readonly(&self) -> bool {
        false
    }

    pub fn set_readonly(&mut self, _readonly: bool) {}
}

impl FileAttr {
    pub fn size(&self) -> u64 {
        0
    }

    pub fn perm(&self) -> FilePermissions {
        FilePermissions
    }

    pub fn file_type(&self) -> FileType {
        FileType
    }

    pub fn modified(&self) -> io::Result<SystemTime> {
        unsupported()
    }

    pub fn accessed(&self) -> io::Result<SystemTime> {
        unsupported()
    }

    pub fn created(&self) -> io::Result<SystemTime> {
        unsupported()
    }
}

impl File {
    pub fn open(_path: &Path, _opts: &OpenOptions) -> io::Result<File> {
        unsupported()
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        unsupported()
    }

    pub fn fsync(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn datasync(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn truncate(&self, _size: u64) -> io::Result<()> {
        unsupported()
    }

    pub fn read(&self, _buf: &mut [u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn read_vectored(&self, _bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        unsupported()
    }

    #[inline]
    pub fn is_read_vectored(&self) -> bool {
        false
    }

    pub fn write(&self, _buf: &[u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn write_vectored(&self, _bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        unsupported()
    }

    #[inline]
    pub fn is_write_vectored(&self) -> bool {
        false
    }

    pub fn seek(&self, _pos: SeekFrom) -> io::Result<u64> {
        unsupported()
    }

    pub fn flush(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn duplicate(&self) -> io::Result<File> {
        unsupported()
    }

    pub fn set_permissions(&self, _perm: FilePermissions) -> io::Result<()> {
        unsupported()
    }
}

pub fn readdir(_p: &Path) -> io::Result<ReadDir> {
    unsupported()
}

pub fn unlink(_path: &Path) -> io::Result<()> {
    unsupported()
}

pub fn rename(_old: &Path, _new: &Path) -> io::Result<()> {
    unsupported()
}

pub fn set_perm(_p: &Path, _perm: FilePermissions) -> io::Result<()> {
    unsupported()
}

pub fn rmdir(_p: &Path) -> io::Result<()> {
    unsupported()
}

pub fn remove_dir_all(_path: &Path) -> io::Result<()> {
    unsupported()
}

pub fn readlink(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}

pub fn symlink(_original: &Path, _link: &Path) -> io::Result<()> {
    unsupported()
}

pub fn link(_original: &Path, _link: &Path) -> io::Result<()> {
    unsupported()
}

pub fn stat(_p: &Path) -> io::Result<FileAttr> {
    unsupported()
}

pub fn lstat(_p: &Path) -> io::Result<FileAttr> {
    unsupported()
}

pub fn canonicalize(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}
