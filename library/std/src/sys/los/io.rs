use crate::{marker::PhantomData, slice};

#[derive(Clone, Copy)]
struct IoBuf {
    buf: *mut u8,
    len: usize,
}

#[derive(Clone, Copy)]
pub struct IoSlice<'a> {
    vec: IoBuf,
    _p: PhantomData<&'a [u8]>,
}

pub struct IoSliceMut<'a> {
    vec: IoBuf,
    _p: PhantomData<&'a mut [u8]>,
}

impl<'a> IoSlice<'a> {
    pub fn new(buf: &'a [u8]) -> IoSlice<'a> {
        IoSlice { vec: IoBuf { buf: buf.as_ptr() as *mut u8, len: buf.len() }, _p: PhantomData }
    }

    pub fn advance(&mut self, n: usize) {
        if (self.vec.len as usize) < n {
            panic!("advancing IoSlice beyond its length");
        }

        unsafe {
            self.vec.len -= n;
            self.vec.buf = self.vec.buf.add(n);
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.vec.buf, self.vec.len) }
    }
}

impl<'a> IoSliceMut<'a> {
    #[inline]
    pub fn new(buf: &'a mut [u8]) -> IoSliceMut<'a> {
        IoSliceMut { vec: IoBuf { len: buf.len(), buf: buf.as_mut_ptr() }, _p: PhantomData }
    }

    #[inline]
    pub fn advance(&mut self, n: usize) {
        if (self.vec.len as usize) < n {
            panic!("advancing IoSliceMut beyond its length");
        }

        unsafe {
            self.vec.len -= n;
            self.vec.buf = self.vec.buf.add(n);
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.vec.buf, self.vec.len) }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.vec.buf, self.vec.len) }
    }
}
