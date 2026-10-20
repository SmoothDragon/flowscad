use derive_more::Neg;

use super::ops::impl_angle;

/// Degrees
///
/// Converts from [`Rad`](crate::Rad), [`Grad`](crate::Grad), and [`Tau`](crate::Tau), but not from a bare number, so an angle's unit is always explicit:
///
/// ```
/// use flowscad::*;
/// let _ = Deg::from(Rad(3.14));
/// let _: Deg = Deg(5.0_f32);
/// ```
///
/// ```compile_fail
/// use flowscad::*;
/// let _ = Deg::from(5);
/// ```
///
/// ```compile_fail
/// use flowscad::*;
/// let _: Deg = 5.0_f32.into();
/// ```
///
/// Adding or subtracting a bare number is rejected for the same reason:
///
/// ```compile_fail
/// use flowscad::*;
/// let _ = Deg(1.) + 5.0_f32;
/// ```
///
/// ```compile_fail
/// use flowscad::*;
/// let _ = Deg(1.) - 5;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Neg)]
pub struct Deg(pub f32);

impl_angle!(Deg, 360.0);

#[cfg(test)]
mod test {
    use std::f32::consts::PI;
    use num_complex::Complex32 as C32;

    use crate::{Deg, Rad};

    #[test]
    fn test_f32_mul_deg() {
        assert_eq!(2.0*Deg(10.), Deg(20.));
    }

    #[test]
    fn test_deg_from_rad() {
        assert_eq!(Deg::from(Rad(PI)), Deg(180.));
    }

    #[test]
    fn test_numeric_mul_deg() {
        assert_eq!(Deg(10.) * 2_i32, Deg(20.));
        assert_eq!(Deg(10.) * 2_u8, Deg(20.));
        assert_eq!(Deg(10.) * 2_usize, Deg(20.));
        assert_eq!(Deg(10.) * 2.0_f64, Deg(20.));
        assert_eq!(2_i64 * Deg(10.), Deg(20.));
        assert_eq!(2_u32 * Deg(10.), Deg(20.));
        assert_eq!(-2_i8 * Deg(10.), Deg(-20.));
    }

    #[test]
    fn test_mul_assign_deg() {
        let mut d = Deg(10.);
        d *= 2;
        d *= 0.5_f64;
        d *= 4_u16;
        assert_eq!(d, Deg(40.));
    }

    #[test]
    fn test_add_deg() {
        assert_eq!(Deg(10.) + Deg(20.), Deg(30.));
        assert_eq!(Deg(90.) + Rad(PI/2.), Deg(180.));
    }

    #[test]
    fn test_add_assign_deg() {
        let mut d = Deg(0.);
        d += Deg(90.);
        d += Rad(PI/2.);
        assert_eq!(d, Deg(180.));
    }

    #[test]
    fn test_sub_deg() {
        assert_eq!(Deg(30.) - Deg(10.), Deg(20.));
        assert_eq!(Deg(180.) - Rad(PI/2.), Deg(90.));
    }

    #[test]
    fn test_sub_assign_deg() {
        let mut d = Deg(180.);
        d -= Deg(45.);
        d -= Rad(PI/4.);
        assert_eq!(d, Deg(90.));
    }

    #[test]
    fn test_neg_deg() {
        assert_eq!(-Deg(10.), Deg(-10.));
    }

    #[test]
    fn test_c32_from_deg() {
        assert_eq!(C32::from(Deg(0.)), C32::new(1., 0.));
        assert_eq!(C32::from(Deg(45.)), C32::new(0.70710677, 0.70710677));
        assert_eq!(C32::from(Deg(90.)), C32::new(-4.371139e-8, 1.0));
        assert_eq!(C32::from(Deg(180.)), C32::new(-1., -8.742278e-8));
    }
}
