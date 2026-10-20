use derive_more::Neg;

use super::ops::impl_angle;

/// Radians
///
/// Converts from [`Deg`](crate::Deg), [`Grad`](crate::Grad), and [`Tau`](crate::Tau), but not from a bare number, so an angle's unit is always explicit:
///
/// ```
/// use flowscad::*;
/// let _: Rad = Deg(180.).into();
/// let _: Rad = Rad(5.0_f32);
/// ```
///
/// ```compile_fail
/// use flowscad::*;
/// let _: Rad = 5.0_f32.into();
/// ```
///
/// ```compile_fail
/// use flowscad::*;
/// let _ = Rad::from(5);
/// ```
///
/// Adding a bare number is rejected for the same reason:
///
/// ```compile_fail
/// use flowscad::*;
/// let _ = Rad(1.) + 5.0_f32;
/// ```
///
/// ```compile_fail
/// use flowscad::*;
/// let _ = Rad(1.) - 5;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Neg)]
pub struct Rad(pub f32);

impl_angle!(Rad, ::std::f32::consts::TAU);

#[cfg(test)]
mod test {
    use std::f32::consts::PI;
    use num_complex::Complex32 as C32;

    use crate::{Deg, Rad};

    #[test]
    fn test_rad_from_deg() {
        assert_eq!(Rad::from(Deg(180.)), Rad(PI));
    }

    #[test]
    fn test_f32_mul_rad() {
        assert_eq!(2.0 * Rad(1.5), Rad(3.));
        assert_eq!(Rad(1.5) * 2.0, Rad(3.));
    }

    #[test]
    fn test_numeric_mul_rad() {
        assert_eq!(Rad(1.5) * 2_i32, Rad(3.));
        assert_eq!(Rad(1.5) * 2_u8, Rad(3.));
        assert_eq!(Rad(1.5) * 2_usize, Rad(3.));
        assert_eq!(Rad(1.5) * 2.0_f64, Rad(3.));
        assert_eq!(2_i64 * Rad(1.5), Rad(3.));
        assert_eq!(2_u32 * Rad(1.5), Rad(3.));
        assert_eq!(-2_i8 * Rad(1.5), Rad(-3.));
    }

    #[test]
    fn test_mul_assign_rad() {
        let mut r = Rad(1.5);
        r *= 2;
        r *= 0.5_f64;
        r *= 4_u16;
        assert_eq!(r, Rad(6.));
    }

    #[test]
    fn test_add_rad() {
        assert_eq!(Rad(1.) + Rad(2.), Rad(3.));
        assert_eq!(Rad(0.) + Deg(180.), Rad(PI));
        assert_eq!(Rad(PI/2.) + Deg(90.), Rad(PI));
    }

    #[test]
    fn test_sub_rad() {
        assert_eq!(Rad(3.) - Rad(1.), Rad(2.));
        assert_eq!(Rad(PI) - Deg(90.), Rad(PI/2.));
    }

    #[test]
    fn test_sub_assign_rad() {
        let mut r = Rad(PI);
        r -= Rad(PI/4.);
        r -= Deg(45.);
        assert!((r.0 - PI/2.).abs() < 1e-6, "{:?}", r);
    }

    #[test]
    fn test_add_assign_rad() {
        let mut r = Rad(0.);
        r += Rad(PI/2.);
        r += Deg(90.);
        assert_eq!(r, Rad(PI));
    }

    #[test]
    fn test_c32_from_rad() {
        assert_eq!(C32::from(Rad(0.)), C32::new(1., 0.));
        assert_eq!(C32::from(Rad(PI/4.)), C32::new(0.70710677, 0.70710677));
        assert_eq!(C32::from(Rad(PI/2.)), C32::new(-4.371139e-8, 1.0));
        assert_eq!(C32::from(Rad(PI)), C32::new(-1., -8.742278e-8));
    }
}
