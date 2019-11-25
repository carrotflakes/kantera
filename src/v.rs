use std::ops::{Add, Sub, Mul, Div, Rem};
use num_traits::Num;
use num_traits::identities::{Zero, One};

pub trait V: Copy + Num + From<f64> {
}

impl<T> V for T
    where T: Copy + Num + From<f64> {
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2<T: Num>(pub T, pub T);
//#[derive(Debug, Clone, Copy)]
//pub struct Vec3<T: Num>(pub T, pub T, pub T);

impl<T: Num> Add for Vec2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl<T: Num> Sub for Vec2<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl<T: Num> Mul for Vec2<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Vec2(self.0 * rhs.0, self.1 * rhs.1)
    }
}
impl<T: Num> Div for Vec2<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Vec2(self.0 / rhs.0, self.1 / rhs.1)
    }
}
impl<T: Num> Rem for Vec2<T> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        Vec2(self.0 % rhs.0, self.1 % rhs.1)
    }
}
impl<T: Num + From<f64>> Zero for Vec2<T> {
    fn zero() -> Self {
        0.0.into()
    }
    fn is_zero(&self) -> bool {
        self.0.is_zero() && self.1.is_zero()
    }
    fn set_zero(&mut self) {
        T::set_zero(&mut self.0);
        T::set_zero(&mut self.1);
    }
}
impl<T: Num + From<f64>> One for Vec2<T> {
    fn one() -> Self {
        1.0.into()
    }
    fn is_one(&self) -> bool {
        self.0.is_one() && self.1.is_one()
    }
    fn set_one(&mut self) {
        T::set_one(&mut self.0);
        T::set_one(&mut self.1);
    }
}
impl<T: Num + From<f64>> From<f64> for Vec2<T> {
    fn from(v: f64) -> Self {
        Vec2(v.into(), v.into())
    }
}
impl<T: Num + From<f64>> Num for Vec2<T> {
    type FromStrRadixErr = ::std::num::ParseIntError;
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        unimplemented!()
    }
}

#[test]
fn test() {
    let v = Vec2(1.0, 2.0);
    assert_eq!(v, Vec2(1.0, 2.0));
    assert_eq!(v, Vec2(2.0, 4.0) / 2.0.into());
}
