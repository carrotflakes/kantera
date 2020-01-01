use std::ops::{Add, Sub, Mul, Div, Rem};
use num_traits::Num;
use num_traits::identities::{Zero, One};

pub trait V: Copy + Num {
    fn lerp(&self, other: &Self, v: f64) -> Self;
    fn bezier(left: &Self, left_handle: &Self, right: &Self, right_handle: &Self, v: f64) -> Self;
}

impl V for f64  {
    #[inline(always)]
    fn lerp(&self, other: &Self, v: f64) -> Self {
        self * (1.0 - v) + other * v
    }
    fn bezier(left: &Self, left_handle: &Self, right: &Self, right_handle: &Self, v: f64) -> Self {
        let iv = 1.0 - v;
        (left + left_handle * v) * iv + (right + right_handle * iv) * v
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2<T: Num + Copy + From<f64>>(pub T, pub T);
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<T: Num + Copy + From<f64>>(pub T, pub T, pub T);

impl<T: Num + Copy + From<f64>> Add for Vec2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl<T: Num + Copy + From<f64>> Sub for Vec2<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl<T: Num + Copy + From<f64>> Mul for Vec2<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Vec2(self.0 * rhs.0, self.1 * rhs.1)
    }
}
impl<T: Num + Copy + From<f64>> Div for Vec2<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Vec2(self.0 / rhs.0, self.1 / rhs.1)
    }
}
impl<T: Num + Copy + From<f64>> Rem for Vec2<T> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        Vec2(self.0 % rhs.0, self.1 % rhs.1)
    }
}
impl<T: Num + Copy + From<f64>> Zero for Vec2<T> {
    fn zero() -> Self {
        Vec2(T::zero(), T::zero())
    }
    fn is_zero(&self) -> bool {
        self.0.is_zero() && self.1.is_zero()
    }
    fn set_zero(&mut self) {
        T::set_zero(&mut self.0);
        T::set_zero(&mut self.1);
    }
}
impl<T: Num + Copy + From<f64>> One for Vec2<T> {
    fn one() -> Self {
        Vec2(T::one(), T::one())
    }
    fn is_one(&self) -> bool {
        self.0.is_one() && self.1.is_one()
    }
    fn set_one(&mut self) {
        T::set_one(&mut self.0);
        T::set_one(&mut self.1);
    }
}
impl<T: Num + Copy + From<f64>> Num for Vec2<T> {
    type FromStrRadixErr = ::std::num::ParseIntError;
    fn from_str_radix(_str: &str, _radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        unimplemented!()
    }
}
impl<T: Num + Copy + From<f64>> V for Vec2<T> {
    #[inline(always)]
    fn lerp(&self, other: &Self, v: f64) -> Self {
        let iv = (1.0 - v).into();
        let v = v.into();
        Vec2(self.0 * iv + other.0 * v, self.1 * iv + other.1 * v)
    }
    fn bezier(left: &Self, left_handle: &Self, right: &Self, right_handle: &Self, v: f64) -> Self {
        let iv = (1.0 - v).into();
        let v = v.into();
        Vec2(
            (left.0 + left_handle.0 * v) * iv + (right.0 + right_handle.0 * iv) * v,
            (left.1 + left_handle.1 * v) * iv + (right.1 + right_handle.1 * iv) * v
        )
    }
}

impl<T: Num + Copy + From<f64>> Add for Vec3<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl<T: Num + Copy + From<f64>> Sub for Vec3<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
impl<T: Num + Copy + From<f64>> Mul for Vec3<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}
impl<T: Num + Copy + From<f64>> Div for Vec3<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}
impl<T: Num + Copy + From<f64>> Rem for Vec3<T> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        Vec3(self.0 % rhs.0, self.1 % rhs.1, self.2 % rhs.2)
    }
}
impl<T: Num + Copy + From<f64>> Zero for Vec3<T> {
    fn zero() -> Self {
        Vec3(T::zero(), T::zero(), T::zero())
    }
    fn is_zero(&self) -> bool {
        self.0.is_zero() && self.1.is_zero() && self.2.is_zero()
    }
    fn set_zero(&mut self) {
        T::set_zero(&mut self.0);
        T::set_zero(&mut self.1);
        T::set_zero(&mut self.2);
    }
}
impl<T: Num + Copy + From<f64>> One for Vec3<T> {
    fn one() -> Self {
        Vec3(T::one(), T::one(), T::one())
    }
    fn is_one(&self) -> bool {
        self.0.is_one() && self.1.is_one() && self.2.is_one()
    }
    fn set_one(&mut self) {
        T::set_one(&mut self.0);
        T::set_one(&mut self.1);
        T::set_one(&mut self.2);
    }
}
impl<T: Num + Copy + From<f64>> Num for Vec3<T> {
    type FromStrRadixErr = ::std::num::ParseIntError;
    fn from_str_radix(_str: &str, _radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        unimplemented!()
    }
}
impl<T: Num + Copy + From<f64>> V for Vec3<T> {
    #[inline(always)]
    fn lerp(&self, other: &Self, v: f64) -> Self {
        let iv = (1.0 - v).into();
        let v = v.into();
        Vec3(self.0 * iv + other.0 * v, self.1 * iv + other.1 * v, self.2 * iv + other.2 * v)
    }
    fn bezier(left: &Self, left_handle: &Self, right: &Self, right_handle: &Self, v: f64) -> Self {
        let iv = (1.0 - v).into();
        let v = v.into();
        Vec3(
            (left.0 + left_handle.0 * v) * iv + (right.0 + right_handle.0 * iv) * v,
            (left.1 + left_handle.1 * v) * iv + (right.1 + right_handle.1 * iv) * v,
            (left.2 + left_handle.2 * v) * iv + (right.2 + right_handle.2 * iv) * v
        )
    }
}

#[test]
fn test() {
    let v = Vec2(1.0, 2.0);
    assert_eq!(v, Vec2(1.0, 2.0));
    assert_eq!(v, Vec2(2.0, 4.0) / Vec2(2.0, 2.0));

    let v = Vec3(0.0, 1.0, 2.0);
    assert_eq!(v, Vec3(0.0, 1.0, 2.0));
    assert_eq!(v + Vec3(0.5, 0.5, 0.5), Vec3(0.5, 1.5, 2.5));
}
