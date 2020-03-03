use crate::lerp::Lerp;
use std::ops::{Add, Sub, Mul, Div, Rem};
use num_traits::Num;
use num_traits::identities::{Zero, One};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2<T: Num + Lerp>(pub T, pub T);
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<T: Num + Lerp>(pub T, pub T, pub T);

impl<T: Num + Lerp> Add for Vec2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl<T: Num + Lerp> Sub for Vec2<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl<T: Num + Lerp> Mul for Vec2<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Vec2(self.0 * rhs.0, self.1 * rhs.1)
    }
}
impl<T: Num + Lerp> Div for Vec2<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Vec2(self.0 / rhs.0, self.1 / rhs.1)
    }
}
impl<T: Num + Lerp> Rem for Vec2<T> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        Vec2(self.0 % rhs.0, self.1 % rhs.1)
    }
}
impl<T: Num + Lerp> Zero for Vec2<T> {
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
impl<T: Num + Lerp> One for Vec2<T> {
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
impl<T: Num + Lerp> Num for Vec2<T> {
    type FromStrRadixErr = ::std::num::ParseIntError;
    fn from_str_radix(_str: &str, _radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        unimplemented!()
    }
}
impl<T: Num + Lerp> Mul<f64> for Vec2<T> {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}
impl<T: Num + Lerp> Lerp for Vec2<T> {}

impl<T: Num + Lerp> Add for Vec3<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl<T: Num + Lerp> Sub for Vec3<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
impl<T: Num + Lerp> Mul for Vec3<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}
impl<T: Num + Lerp> Div for Vec3<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}
impl<T: Num + Lerp> Rem for Vec3<T> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        Vec3(self.0 % rhs.0, self.1 % rhs.1, self.2 % rhs.2)
    }
}
impl<T: Num + Lerp> Zero for Vec3<T> {
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
impl<T: Num + Lerp> One for Vec3<T> {
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
impl<T: Num + Lerp> Num for Vec3<T> {
    type FromStrRadixErr = ::std::num::ParseIntError;
    fn from_str_radix(_str: &str, _radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        unimplemented!()
    }
}
impl<T: Num + Lerp> Mul<f64> for Vec3<T> {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}
impl<T: Num + Lerp> Lerp for Vec3<T> {}

#[test]
fn test() {
    let v = Vec2(1.0, 2.0);
    assert_eq!(v, Vec2(1.0, 2.0));
    assert_eq!(v, Vec2(2.0, 4.0) / Vec2(2.0, 2.0));

    let v = Vec3(0.0, 1.0, 2.0);
    assert_eq!(v, Vec3(0.0, 1.0, 2.0));
    assert_eq!(v + Vec3(0.5, 0.5, 0.5), Vec3(0.5, 1.5, 2.5));
}
