use derive_more::Neg;

use super::ops::impl_angle;

/// Gradians (400 per full turn)
///
/// Converts from [`Rad`](crate::Rad), [`Deg`](crate::Deg), and [`Tau`](crate::Tau), but not from a bare number, so an angle's unit is always explicit:
///
/// ```
/// use flowscad::*;
/// let _ = Grad::from(Rad(3.14));
/// let _: Grad = Deg(90.).into();
/// let _: Grad = Grad(5.0_f32);
/// ```
///
/// ```compile_fail
/// use flowscad::*;
/// let _ = Grad::from(5);
/// ```
///
/// ```compile_fail
/// use flowscad::*;
/// let _: Grad = 5.0_f32.into();
/// ```
///
/// Adding or subtracting a bare number is rejected for the same reason:
///
/// ```compile_fail
/// use flowscad::*;
/// let _ = Grad(1.) + 5.0_f32;
/// ```
///
/// ```compile_fail
/// use flowscad::*;
/// let _ = Grad(1.) - 5;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Neg)]
pub struct Grad(pub f32);

impl_angle!(Grad, 400.0);

#[cfg(test)]
mod test {
    use std::f32::consts::PI;
    use num_complex::Complex32 as C32;

    use crate::{Deg, Grad, Rad};

    #[test]
    fn test_grad_from_rad() {
        assert_eq!(Grad::from(Rad(PI)), Grad(200.));
    }

    #[test]
    fn test_grad_from_deg() {
        assert_eq!(Grad::from(Deg(90.)), Grad(100.));
        assert_eq!(Grad::from(Deg(360.)), Grad(400.));
    }

    #[test]
    fn test_rad_from_grad() {
        assert_eq!(Rad::from(Grad(200.)), Rad(PI));
    }

    #[test]
    fn test_deg_from_grad() {
        assert_eq!(Deg::from(Grad(100.)), Deg(90.));
        assert_eq!(Deg::from(Grad(400.)), Deg(360.));
    }

    #[test]
    fn test_numeric_mul_grad() {
        assert_eq!(Grad(10.) * 2_i32, Grad(20.));
        assert_eq!(Grad(10.) * 2_u8, Grad(20.));
        assert_eq!(Grad(10.) * 2.0_f64, Grad(20.));
        assert_eq!(2_usize * Grad(10.), Grad(20.));
        assert_eq!(-2_i8 * Grad(10.), Grad(-20.));
    }

    #[test]
    fn test_mul_assign_grad() {
        let mut g = Grad(10.);
        g *= 2;
        g *= 0.5_f64;
        g *= 4_u16;
        assert_eq!(g, Grad(40.));
    }

    #[test]
    fn test_add_grad() {
        assert_eq!(Grad(10.) + Grad(20.), Grad(30.));
        assert_eq!(Grad(100.) + Deg(90.), Grad(200.));
        assert_eq!(Grad(100.) + Rad(PI/2.), Grad(200.));
    }

    #[test]
    fn test_add_assign_grad() {
        let mut g = Grad(0.);
        g += Grad(100.);
        g += Deg(90.);
        g += Rad(PI);
        assert_eq!(g, Grad(400.));
    }

    #[test]
    fn test_sub_grad() {
        assert_eq!(Grad(30.) - Grad(10.), Grad(20.));
        assert_eq!(Grad(200.) - Deg(90.), Grad(100.));
        assert_eq!(Grad(200.) - Rad(PI/2.), Grad(100.));
    }

    #[test]
    fn test_sub_assign_grad() {
        let mut g = Grad(400.);
        g -= Grad(100.);
        g -= Deg(90.);
        g -= Rad(PI/2.);
        assert_eq!(g, Grad(100.));
    }

    #[test]
    fn test_neg_grad() {
        assert_eq!(-Grad(10.), Grad(-10.));
    }

    #[test]
    fn test_mixed_units_into_grad() {
        assert_eq!(Deg(0.) + Grad(100.), Deg(90.));
        assert_eq!(Rad(0.) + Grad(200.), Rad(PI));
    }

    #[test]
    fn test_c32_from_grad() {
        assert_eq!(C32::from(Grad(0.)), C32::new(1., 0.));
        assert_eq!(C32::from(Grad(50.)), C32::new(0.70710677, 0.70710677));
        assert_eq!(C32::from(Grad(100.)), C32::new(-4.371139e-8, 1.0));
        assert_eq!(C32::from(Grad(200.)), C32::new(-1., -8.742278e-8));
    }
}
