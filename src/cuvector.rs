
use std;
use std::ops::{Deref, DerefMut};
use libc::c_void;

use ffi::cuda_ffi::*;
use ffi::kernel_ffi::*;

use assert::*;



pub struct CuVector {
    pub(super) len: usize,
    pub(super) data: *mut f32,
}
impl Drop for CuVector {
    fn drop(&mut self) {
        unsafe { cudaFree(self.data as *mut c_void) }.assert_success();
    }
}
impl CuVector {
    pub fn new(len: usize) -> CuVector {
        let mut data = std::ptr::null_mut();
        unsafe { cudaMalloc(&mut data, len*std::mem::size_of::<f32>()) }.assert_success();
        CuVector { len, data: (data as *mut f32) }
    }
    pub fn from_data(data: &[f32]) -> CuVector {
        let mut output = Self::new(data.len());
        output.copy_from_host(data);
        output
    }

    #[inline]
    pub fn len(&self) -> usize {
        return self.len;
    }
    pub fn slice(&self, offset: usize, len: usize) -> CuVectorSlice {
        assert!(offset < self.len);
        assert!(len <= self.len-offset);
        CuVectorSlice {
            _parent: self,
            value: CuVector { len, data: unsafe { self.data.offset(offset as isize) } }
        }
    }
    pub fn slice_mut(&mut self, offset: usize, len: usize) -> CuVectorSliceMut {
        assert!(offset < self.len);
        assert!(len <= self.len-offset);
        CuVectorSliceMut {
            _parent: self,
            value: CuVector { len, data: unsafe { self.data.offset(offset as isize) } }
        }
    }

    pub fn init(&mut self, value: f32) {
        unsafe { CudaKernel_vectorSet((self.data as *mut f32), (self.len as i32), value) }
    }
    pub fn copy_from_host(&mut self, data: &[f32]) {
        assert_eq_usize(self.len, "self.len()", data.len(), "data.len()");
        unsafe {
            cudaMemcpy((self.data as *mut c_void), (data.as_ptr() as *const c_void), self.len*std::mem::size_of::<f32>(), CudaMemcpyKind::HostToDevice)
        }.assert_success();
    }
    pub fn copy_to_host(&self, data: &mut [f32]) {
        assert_eq_usize(self.len, "self.len()", data.len(), "data.len()");
        unsafe {
            cudaMemcpy((data.as_mut_ptr() as *mut c_void), (self.data as *mut c_void),
                       self.len*std::mem::size_of::<f32>(), CudaMemcpyKind::DeviceToHost)
        }.assert_success();
    }

    pub fn copy_from_device(&mut self, source: &CuVector) {
        assert_eq_usize(self.len, "self.len()", source.len, "source.len()");
        unsafe {
            cudaMemcpy((self.data as *mut c_void), (source.data as *mut c_void),
                       self.len*std::mem::size_of::<f32>(), CudaMemcpyKind::DeviceToDevice)
        }.assert_success();
    }

    pub fn add_scl_self(&mut self, value: f32) {
        unsafe { CudaKernel_vectorAddSclSelf((self.data as *mut f32), (self.len as i32), value) }
    }
    pub fn scale_self(&mut self, value: f32) {
        unsafe { CudaKernel_vectorScaleSelf(self.data, (self.len as i32), value) }
    }
    pub fn add_self(&mut self, right_op: &Self) {
        assert_eq_usize(self.len, "self.len()", right_op.len, "right_op.len()");
        unsafe { CudaKernel_vectorAdd(self.data, right_op.data, self.data, self.len as i32) }
    }
    pub fn pmult_self(&mut self, right_op: &Self) {
        assert_eq_usize(self.len, "self.len()", right_op.len, "right_op.len()");
        unsafe { CudaKernel_vectorPMult(self.data, right_op.data, self.data, (self.len as i32)) }
    }

    pub fn sub(left_op: &Self, right_op: &Self, output: &mut Self) {
        assert_eq_usize(left_op.len, "left_op.len()", right_op.len, "right_op.len()");
        assert_eq_usize(left_op.len, "left_op.len()", output.len, "output.len()");
        unsafe { CudaKernel_vectorSub((left_op.data as *const f32),
                                      (right_op.data as *const f32),
                                      (output.data as *mut f32),
                                      (left_op.len as i32)) }
    }
    pub fn pmult(left_op: &Self, right_op: &Self, output: &mut Self) {
        assert_eq_usize(left_op.len, "left_op.len()", right_op.len, "right_op.len()");
        assert_eq_usize(left_op.len, "left_op.len()", output.len, "output.len()");
        unsafe { CudaKernel_vectorPMult((left_op.data as *const f32),
                                        (right_op.data as *const f32),
                                        (output.data as *mut f32),
                                        (left_op.len as i32)) }
    }
    pub fn sigmoid(vector: &Self, output: &mut Self) {
        assert_eq_usize(vector.len, "vector.len()", output.len, "output.len()");
        unsafe { CudaKernel_vectorSigmoid((vector.data as *const f32), (output.data as *mut f32), (vector.len as i32)) }
    }
    pub fn sigmoid_deriv(vector: &Self, output: &mut Self) {
        assert_eq_usize(vector.len, "vector.len()", output.len, "output.len()");
        unsafe { CudaKernel_vectorSigmoidDeriv((vector.data as *const f32), (output.data as *mut f32), (vector.len as i32)) }
    }

    #[allow(dead_code)]
    pub fn dev_print(&self, msg: &str) {
        let mut buffer = vec![0.0; self.len];
        self.copy_to_host(&mut buffer);
        print!("{}   ", msg);
        for i in 0..self.len {
            print!("{}, ", buffer[i])
        }
        println!()
    }
}

pub struct CuVectorSlice<'a> {
    _parent: &'a CuVector,
    value: CuVector,
}
impl<'a> Drop for CuVectorSlice<'a> {
    fn drop(&mut self) {
        self.value.data = std::ptr::null_mut();
    }
}
impl<'a> Deref for CuVectorSlice<'a> {
    type Target = CuVector;
    fn deref(&self) -> &CuVector {
        &self.value
    }
}

pub struct CuVectorSliceMut<'a> {
    _parent: &'a CuVector,
    value: CuVector,
}
impl<'a> Drop for CuVectorSliceMut<'a> {
    fn drop(&mut self) {
        self.value.data = std::ptr::null_mut();
    }
}
impl<'a> Deref for CuVectorSliceMut<'a> {
    type Target = CuVector;
    fn deref(&self) -> &CuVector {
        &self.value
    }
}
impl<'a> DerefMut for CuVectorSliceMut<'a> {
    fn deref_mut(&mut self) -> &mut CuVector {
        &mut self.value
    }
}



#[cfg(test)]
mod tests {
    fn assert_equals_float(a: f32, b: f32) {
        let d = a-b;
        if d < -0.000001 || d > 0.000001 {
            panic!("{} != {}", a, b);
        }
    }

    #[test]
    fn init() {
        let value0 = -0.23254;
        let value1 = 1.1852;
        let mut vector = super::CuVector::new(5);

        vector.init(value0);
        vector.slice_mut(1, 3).init(value1);

        let mut output = vec![0.0; 5];
        vector.copy_to_host(&mut output);

        assert_equals_float(output[0], value0);
        assert_equals_float(output[1], value1);
        assert_equals_float(output[2], value1);
        assert_equals_float(output[3], value1);
        assert_equals_float(output[4], value0);
    }
    #[test]
    fn add_self() {
        let value0 = -0.23254;
        let value1 = 1.185254;
        let mut vector0 = super::CuVector::new(5);
        let mut vector1 = super::CuVector::new(2);

        vector0.init(value0);
        vector1.init(value1);
        vector0.slice_mut(2, 2).add_self(&vector1);

        let mut output = vec![0.0; 5];
        vector0.copy_to_host(&mut output);

        assert_equals_float(output[0], value0);
        assert_equals_float(output[1], value0);
        assert_equals_float(output[2], value0+value1);
        assert_equals_float(output[3], value0+value1);
        assert_equals_float(output[4], value0);
    }
}