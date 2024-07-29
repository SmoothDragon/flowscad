use nalgebra as na;
use itertools::Itertools;
use std::ops::*;

use derive_more::*;

#[derive(Clone, Copy, PartialEq, Add, Sub, Neg)]
pub struct Real(pub f32);

impl Real {
    /// Positive Real MAX is lower since it is used for super large objects that could be shifted or rotated.
    pub const MAX: Real = Real(f32::MAX/1000.0);
}

impl std::fmt::Debug for Real {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl std::fmt::Display for Real {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl std::ops::AddAssign for Real {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0);
    }
}

impl std::ops::SubAssign for Real {
    fn sub_assign(&mut self, other: Self) {
        *self = Self(self.0 - other.0);
    }
}

// TODO: Macro to replace all this?
impl From<u32> for Real {
    fn from(i: u32) -> Real {
        Real(i as f32)
    }
}

impl From<i32> for Real {
    fn from(i: i32) -> Real {
        Real(i as f32)
    }
}

impl From<u64> for Real {
    fn from(i: u64) -> Real {
        Real(i as f32)
    }
}

impl From<i64> for Real {
    fn from(i: i64) -> Real {
        Real(i as f32)
    }
}

impl From<f32> for Real {
    fn from(f: f32) -> Real {
        Real(f as f32)
    }
}

impl From<f64> for Real {
    fn from(f: f64) -> Real {
        Real(f as f32)
    }
}

impl<X: Into<Real>> std::ops::Mul<X> for Real {
    type Output = Real;
    fn mul(self, other: X) -> Self::Output {
        Real(self.0 * other.into().0)
    }
}

impl std::ops::Mul<Real> for f32 {
    type Output = Real;
    fn mul(self, other: Real) -> Real {
        Real(self * other.0)
    }
}

impl std::ops::Mul<Real> for i32 {
    type Output = Real;
    fn mul(self, other: Real) -> Real {
        Real((self as f32) * other.0)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Add, Neg)]
pub struct XY(pub f32, pub f32); 

impl std::fmt::Display for XY {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", format!("[{}, {}]", &self.0, &self.1))
        // write!(f, "{}", format!("{:?}", &self.0).replace(r"[[", r"[").replace("]]", "]"))
    }
}

pub fn v2<X: Into<Real>, Y: Into<Real>>(x: X, y: Y) -> XY {
    XY(x.into().0, y.into().0)
}

impl<X: Into<Real>, Y: Into<Real>> From<(X,Y)> for XY {
    fn from(xy: (X,Y)) -> XY {
        v2(xy.0, xy.1)
    }
}

/// Generalized multiplication on the right is possible
impl<X: Into<Real>> std::ops::Mul<X> for XY {
    type Output = XY;
    fn mul(self, other: X) -> Self::Output {
        let y: f32 = other.into().0;
        // v2(self.0.x * other.clone().into().0, self.0.y * other.into().0)
        v2(self.0 * y, self.1 * y)
    }
}

/// Generalized multiplication on the left is not currently possible
/// Each type must be specified individually
/// TODO: This should become a macro
impl std::ops::Mul<XY> for f32 {
    type Output = XY;
    fn mul(self, rhs: XY) -> Self::Output {
        v2(rhs.0 * self as f32, rhs.1 * self as f32)
    }
}

impl std::ops::Mul<XY> for i32 {
    type Output = XY;
    fn mul(self, rhs: XY) -> Self::Output {
        v2(rhs.0 * self as f32, rhs.1 * self as f32)
    }
}


impl Sub for XY {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

/// Multiplication treats XY as a complex number
impl Mul for XY {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0 - self.1 * other.1, 
             self.0 * other.1 + self.1 * other.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Add)]
pub struct XYZ(pub na::Vector3<Real>);  // TODO: Remove pub na::

impl std::fmt::Display for XYZ {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", &self.0).replace(r"[[", r"[").replace("]]", "]"))
    }
}

pub fn v3<X: Into<Real>, Y: Into<Real>, Z: Into<Real>>(x: X, y: Y, z: Z) -> XYZ {
    XYZ(nalgebra::vector![x.into(), y.into(), z.into()])
}

impl From<XY> for XYZ {
    fn from(xy: XY) -> XYZ {
        v3(xy.0, xy.1, 0)
    }
}

impl std::ops::Mul<f32> for XYZ {
    type Output = XYZ;
    fn mul(self, rhs: f32) -> Self::Output {
        v3(self.0.x * rhs, self.0.y * rhs, self.0.z * rhs)
    }
}


impl std::ops::Mul<XYZ> for f32 {
    type Output = XYZ;
    fn mul(self, rhs: XYZ) -> Self::Output {
        v3(rhs.0.x * self, rhs.0.y * self, rhs.0.z * self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_i32() {
        assert_eq!(Real::from(5_i32), Real(5.));
        assert_eq!(Real::from(i32::MAX), Real(2147483600.0));
    }

    #[test]
    fn test_into_real() {
        assert_eq!(<i32 as Into<Real>>::into(5), Real(5.));
        assert_eq!(<u32 as Into<Real>>::into(5), Real(5.));
        assert_eq!(<f64 as Into<Real>>::into(5.0), Real(5.));
    }

    #[test]
    fn test_real_mul() {
        assert_eq!(Real(5.) * 2., Real(10.));
        assert_eq!(Real(5.) * 2, Real(10.));
        assert_eq!(2. * Real(5.), Real(10.));
        assert_eq!(2 * Real(5.), Real(10.));
    }

    #[test]
    fn test_v2_mul() {
        assert_eq!(format!("{}", v2(1.,2.)*3.), "[3, 6]");
        assert_eq!(format!("{}", 3. * v2(1.,2.)), "[3, 6]");
        assert_eq!(format!("{}", v2(1.,2.)*3), "[3, 6]");
        assert_eq!(format!("{}", 3 * v2(1.,2.)), "[3, 6]");
    }

    #[test]
    fn test_v2_v2_mul() {
        assert_eq!(format!("{}", v2(1.,2.)*v2(1, -2)), "[5, 0]");
    }

    #[test]
    fn test_v3_mul() {
        assert_eq!(format!("{}", v3(1.,2., 4)*3.), "[3, 6, 12]");
        assert_eq!(format!("{}", 8. * v3(1.,2., 4)), "[8, 16, 32]");
    }

    #[test]
    fn test_into_real2() {
        assert_eq!(XY::from( (5_i32, 10_i32) ), v2(5., 10.));
        assert_eq!(XY::from( (5_i32, 10_u64) ), v2(5., 10.));
        assert_eq!(XY::from( (5.0_f32, 10_u64) ), v2(5., 10.));
    }

}
