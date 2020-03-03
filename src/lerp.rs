use std::ops::{Add, Mul};

pub trait Lerp: Copy + Add<Output = Self> + Mul<f64, Output = Self> {
    #[inline(always)]
    fn lerp(&self, other: &Self, v: f64) -> Self {
        *self * (1.0 - v) + *other * v
    }
}

impl Lerp for f64 {}
