use derive_more::Neg;

use super::ops::impl_angle;

/// Turns, where one Tau is a complete turn (2π radians)
///
/// Converts from [`Rad`](crate::Rad), [`Deg`](crate::Deg), and [`Grad`](crate::Grad), but not from a bare number, so an angle's unit is always explicit:
///
/// ```
/// use flowscad::*;
/// let _ = Tau::from(Rad(3.14));
/// let _: Tau = Deg(90.).into();
/// let _: Tau = Tau(0.5_f32);
/// ```
///
/// ```compile_fail
/// use flowscad::*;
/// let _ = Tau::from(5);
/// ```
///
/// ```compile_fail
/// use flowscad::*;
/// let _: Tau = 5.0_f32.into();
/// ```
///
/// Adding or subtracting a bare number is rejected for the same reason:
///
/// ```compile_fail
/// use flowscad::*;
/// let _ = Tau(1.) + 5.0_f32;
/// ```
///
/// ```compile_fail
/// use flowscad::*;
/// let _ = Tau(1.) - 5;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Neg)]
pub struct Tau(pub f32);

impl_angle!(Tau, 1.0);

#[cfg(test)]
mod test {
    use std::f32::consts::{PI, TAU};
    use num_complex::Complex32 as C32;

    use crate::{Deg, Grad, Rad, Tau};

    #[test]
    fn test_tau_from_other_units() {
        assert_eq!(Tau::from(Rad(TAU)), Tau(1.));
        assert_eq!(Tau::from(Rad(PI)), Tau(0.5));
        assert_eq!(Tau::from(Deg(90.)), Tau(0.25));
        assert_eq!(Tau::from(Grad(100.)), Tau(0.25));
    }

    #[test]
    fn test_other_units_from_tau() {
        assert_eq!(Rad::from(Tau(1.)), Rad(TAU));
        assert_eq!(Rad::from(Tau(0.5)), Rad(PI));
        assert_eq!(Deg::from(Tau(0.25)), Deg(90.));
        assert_eq!(Grad::from(Tau(0.25)), Grad(100.));
    }

    #[test]
    fn test_numeric_mul_tau() {
        assert_eq!(Tau(0.25) * 2_i32, Tau(0.5));
        assert_eq!(Tau(0.25) * 2.0_f64, Tau(0.5));
        assert_eq!(4_usize * Tau(0.25), Tau(1.));
        assert_eq!(-2_i8 * Tau(0.25), Tau(-0.5));
    }

    #[test]
    fn test_mul_assign_tau() {
        let mut t = Tau(0.125);
        t *= 2;
        t *= 4_u16;
        assert_eq!(t, Tau(1.));
    }

    #[test]
    fn test_add_sub_tau() {
        assert_eq!(Tau(0.25) + Tau(0.5), Tau(0.75));
        assert_eq!(Tau(0.25) + Deg(90.), Tau(0.5));
        assert_eq!(Tau(0.5) + Rad(PI), Tau(1.));
        assert_eq!(Tau(1.) - Grad(100.), Tau(0.75));
        assert_eq!(Deg(0.) + Tau(0.5), Deg(180.));
    }

    #[test]
    fn test_add_sub_assign_tau() {
        let mut t = Tau(0.);
        t += Deg(180.);
        t += Tau(0.5);
        t -= Grad(100.);
        assert_eq!(t, Tau(0.75));
    }

    #[test]
    fn test_neg_tau() {
        assert_eq!(-Tau(0.25), Tau(-0.25));
    }

    #[test]
    fn test_trig_tau() {
        assert!((Tau(0.25).sin() - 1.).abs() < 1e-6);
        assert!((Tau(0.5).cos() + 1.).abs() < 1e-6);
        assert!((Tau(0.125).tan() - 1.).abs() < 1e-6);
        assert_eq!(Tau::asin(1.), Tau(0.25));
        assert_eq!(Tau::acos(-1.), Tau(0.5));
        assert_eq!(Tau::atan2(1., 1.), Tau(0.125));
    }

    #[test]
    fn test_c32_from_tau() {
        assert_eq!(C32::from(Tau(0.)), C32::new(1., 0.));
        assert_eq!(C32::from(Tau(0.25)), C32::new(-4.371139e-8, 1.0));
        assert_eq!(C32::from(Tau(0.5)), C32::new(-1., -8.742278e-8));
    }
}
