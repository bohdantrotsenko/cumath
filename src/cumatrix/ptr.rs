
use cuda_core::cuda::*;


/// A pointer over a matrix : It won't free the inner GPU-pointer when it goes out of scope
pub struct CuMatrixPtr {
    pub(crate) deref: CuMatrixPtrDeref,
}
impl CuMatrixPtr {

    /// [inline]
    /// Returns the number of rows of the underlying vector (even if the pointed memory has been freed)
    #[inline]
    pub fn rows(&self) -> usize {
        self.deref.rows
    }

    /// [inline]
    /// Returns the number of columns of the underlying vector (even if the pointed memory has been freed)
    #[inline]
    pub fn cols(&self) -> usize {
        self.deref.cols
    }

    /// [inline]
    /// Returns the length of the underlying vector (even if the pointed memory has been freed)
    #[inline]
    pub fn len(&self) -> usize {
        self.deref.len
    }

    /// [inline]
    /// Returns the underlying vector
    #[inline]
    pub unsafe fn deref(&mut self) -> &mut CuMatrixPtrDeref {
        &mut self.deref
    }

}


pub struct CuMatrixPtrDeref {
    pub(crate) rows: usize,
    pub(crate) cols: usize,
    pub(crate) len: usize,
    pub(crate) ptr: *mut f32,
}
impl_CuPackedDataMut!(CuMatrixPtrDeref);
impl_CuMatrixOpMut_packed!(CuMatrixPtrDeref);