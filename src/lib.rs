mod meta;
mod cuda_core;
mod cuvector;
mod cumatrix;
mod kernel;

pub use cuda_core::{cuda::*, cublas::*, curand::*};
pub use cuvector::*;
pub use cumatrix::*;


use std::fmt::Display;


pub trait CuDataType: Clone + PartialEq + Display {
    fn zero() -> Self;
}
impl CuDataType for i32 {
    fn zero() -> i32 { 0 }
}
impl CuDataType for f32 {
    fn zero() -> f32 { 0.0 }
}

