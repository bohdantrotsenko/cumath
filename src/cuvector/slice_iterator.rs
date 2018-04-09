
use super::*;


/// An iterator over a vector, returning vector slices.
pub struct CuVectorSliceIter<'a> {
    pub(super) parent: PhantomData<&'a CuVectorOp>,
    pub(super) len: usize,
    pub(super) ptr: *const f32,
}
impl<'a> CuVectorSliceIter<'a> {

    pub fn next<'b, 'c>(&'c mut self, len: usize) -> Option<CuVectorSlice<'b>> where 'a: 'b, 'b: 'c {
        match len <= self.len {
            true => {
                let ptr = self.ptr;
                self.ptr = unsafe { self.ptr.offset(len as isize) };
                self.len -= len;
                Some(CuVectorSlice { parent: PhantomData, len, ptr })
            }
            false => None
        }
    }

    pub fn skip(&mut self, len: usize) {
        if len <= self.len {
            self.ptr = unsafe { self.ptr.offset(len as isize) };
            self.len -= len;
        } else {
            self.len = 0;
        }
    }

}




/// An iterator over a mutable vector, returning mutable vector slices.
pub struct CuVectorSliceMutIter<'a> {
    pub(super) parent: PhantomData<&'a CuVectorOpMut>,
    pub(super) len: usize,
    pub(super) ptr: *mut f32,
}
impl<'a> CuVectorSliceMutIter<'a> {

    pub fn next<'b, 'c>(&'c mut self, len: usize) -> Option<CuVectorSliceMut<'b>> where 'a: 'b, 'b: 'c {
        match len <= self.len {
            true => {
                let ptr = self.ptr;
                self.ptr = unsafe { self.ptr.offset(len as isize) };
                self.len -= len;
                Some(CuVectorSliceMut { parent: PhantomData, len, ptr })
            }
            false => None
        }
    }

    pub fn skip(&mut self, len: usize) {
        if len <= self.len {
            self.ptr = unsafe { self.ptr.offset(len as isize) };
            self.len -= len;
        } else {
            self.len = 0;
        }
    }

}


#[cfg(test)]
mod tests {

    /*use vector::{CuVectorOp, CuVectorOpMut};

    #[test]
    fn test() {

        let mut vector = super::CuVector::new(10, 0.0);
        //let mut iter = vector.slice_mut_iter();
        let mut iter = vector.slice_iter();
        let mut iter = vector.slice_iter();
        let mut iter = vector.slice_iter();

        let _slice1 = iter.next(2).unwrap();
        let _slice2 = iter.next(2).unwrap();

        /*let slice1 = iter.next2(2).unwrap();
        iter.next2(2).unwrap();*/

    }*/

}