use crate::{Deg, Grad, Rad, Tau};

/// Defines everything an angle type needs from its size of one full turn: `PER_TURN`,
/// conversion to a unit complex number, arithmetic, and trigonometry.
///
/// Addition and subtraction accept anything that converts into the angle type, so units can be
/// mixed. Multiplication accepts any primitive number, but there is deliberately no
/// `From<number>`, so a bare number can never be added to an angle.
macro_rules! impl_angle {
    ($angle:ident, $per_turn:expr) => {
        impl $angle {
            /// Number of units in one full turn.
            pub const PER_TURN: f32 = $per_turn;

            #[inline]
            fn radians(self) -> f32 {
                $crate::Rad::from(self).0
            }

            #[inline]
            pub fn sin(self) -> f32 {
                self.radians().sin()
            }

            #[inline]
            pub fn cos(self) -> f32 {
                self.radians().cos()
            }

            #[inline]
            pub fn tan(self) -> f32 {
                self.radians().tan()
            }

            /// Returns `(sin, cos)`.
            #[inline]
            pub fn sin_cos(self) -> (f32, f32) {
                self.radians().sin_cos()
            }

            /// Angle whose sine is `x`, in `[-quarter turn, quarter turn]`.
            #[inline]
            pub fn asin(x: f32) -> Self {
                $crate::Rad(x.asin()).into()
            }

            /// Angle whose cosine is `x`, in `[0, half turn]`.
            #[inline]
            pub fn acos(x: f32) -> Self {
                $crate::Rad(x.acos()).into()
            }

            /// Angle whose tangent is `x`, in `[-quarter turn, quarter turn]`.
            #[inline]
            pub fn atan(x: f32) -> Self {
                $crate::Rad(x.atan()).into()
            }

            /// Angle of the point `(x, y)` from the positive x axis, in `[-half turn, half turn]`.
            #[inline]
            pub fn atan2(y: f32, x: f32) -> Self {
                $crate::Rad(y.atan2(x)).into()
            }
        }

        impl From<$angle> for ::num_complex::Complex32 {
            fn from(angle: $angle) -> Self {
                let (sin, cos) = angle.sin_cos();
                Self::new(cos, sin)
            }
        }

        impl<T: Into<$angle>> ::std::ops::Add<T> for $angle {
            type Output = $angle;
            #[inline]
            fn add(self, rhs: T) -> Self::Output {
                $angle(self.0 + rhs.into().0)
            }
        }

        impl<T: Into<$angle>> ::std::ops::AddAssign<T> for $angle {
            #[inline]
            fn add_assign(&mut self, rhs: T) {
                self.0 += rhs.into().0;
            }
        }

        impl<T: Into<$angle>> ::std::ops::Sub<T> for $angle {
            type Output = $angle;
            #[inline]
            fn sub(self, rhs: T) -> Self::Output {
                $angle(self.0 - rhs.into().0)
            }
        }

        impl<T: Into<$angle>> ::std::ops::SubAssign<T> for $angle {
            #[inline]
            fn sub_assign(&mut self, rhs: T) {
                self.0 -= rhs.into().0;
            }
        }

        $crate::angle::ops::impl_angle!(@mul $angle;
            f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
    };

    (@mul $angle:ident; $($t:ty),*) => {
        $(
            impl ::std::ops::Mul<$t> for $angle {
                type Output = $angle;
                #[inline]
                fn mul(self, rhs: $t) -> Self::Output {
                    $angle(self.0 * rhs as f32)
                }
            }

            impl ::std::ops::Mul<$angle> for $t {
                type Output = $angle;
                #[inline]
                fn mul(self, rhs: $angle) -> Self::Output {
                    $angle(self as f32 * rhs.0)
                }
            }

            impl ::std::ops::MulAssign<$t> for $angle {
                #[inline]
                fn mul_assign(&mut self, rhs: $t) {
                    self.0 *= rhs as f32;
                }
            }
        )*
    };
}

/// Implements `From` in both directions between every pair of listed angle types.
macro_rules! impl_angle_conversions {
    () => {};
    ($first:ident $(, $rest:ident)*) => {
        $(
            impl From<$rest> for $first {
                fn from(angle: $rest) -> Self {
                    $first(angle.0 * $first::PER_TURN / $rest::PER_TURN)
                }
            }

            impl From<$first> for $rest {
                fn from(angle: $first) -> Self {
                    $rest(angle.0 * $rest::PER_TURN / $first::PER_TURN)
                }
            }
        )*
        impl_angle_conversions!($($rest),*);
    };
}

pub(crate) use impl_angle;

impl_angle_conversions!(Rad, Deg, Grad, Tau);

#[cfg(test)]
mod test {
    use std::f32::consts::{PI, TAU};

    use crate::{Deg, Grad, Rad, Tau};

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn test_sin() {
        assert!(close(Rad(PI/6.).sin(), 0.5));
        assert!(close(Deg(30.).sin(), 0.5));
        assert!(close(Grad(100./3.).sin(), 0.5));
        assert!(close(Deg(90.).sin(), 1.));
        assert!(close(Deg(-90.).sin(), -1.));
        assert!(close(Grad(200.).sin(), 0.));
    }

    #[test]
    fn test_cos() {
        assert!(close(Rad(PI/3.).cos(), 0.5));
        assert!(close(Deg(60.).cos(), 0.5));
        assert!(close(Grad(200./3.).cos(), 0.5));
        assert!(close(Deg(180.).cos(), -1.));
        assert!(close(Grad(0.).cos(), 1.));
    }

    #[test]
    fn test_tan() {
        assert!(close(Rad(PI/4.).tan(), 1.));
        assert!(close(Deg(45.).tan(), 1.));
        assert!(close(Grad(50.).tan(), 1.));
        assert!(close(Deg(-45.).tan(), -1.));
    }

    #[test]
    fn test_sin_cos() {
        let (s, c) = Deg(30.).sin_cos();
        assert!(close(s, 0.5) && close(c, 3f32.sqrt() / 2.));
        assert_eq!(Rad(1.).sin_cos(), (Rad(1.).sin(), Rad(1.).cos()));
        assert_eq!(Grad(37.).sin_cos(), (Grad(37.).sin(), Grad(37.).cos()));
    }

    #[test]
    fn test_asin() {
        assert!(close(Rad::asin(1.).0, PI/2.));
        assert_eq!(Deg::asin(1.), Deg(90.));
        assert_eq!(Grad::asin(1.), Grad(100.));
        assert!(close(Deg::asin(0.5).0, 30.));
        assert_eq!(Deg::asin(-1.), Deg(-90.));
    }

    #[test]
    fn test_acos() {
        assert!(close(Rad::acos(-1.).0, PI));
        assert_eq!(Deg::acos(-1.), Deg(180.));
        assert_eq!(Grad::acos(-1.), Grad(200.));
        assert_eq!(Deg::acos(1.), Deg(0.));
    }

    #[test]
    fn test_atan() {
        assert!(close(Rad::atan(1.).0, PI/4.));
        assert_eq!(Deg::atan(1.), Deg(45.));
        assert_eq!(Grad::atan(1.), Grad(50.));
    }

    #[test]
    fn test_atan2_quadrants() {
        assert_eq!(Deg::atan2(1., 1.), Deg(45.));
        assert_eq!(Deg::atan2(1., -1.), Deg(135.));
        assert_eq!(Deg::atan2(-1., -1.), Deg(-135.));
        assert_eq!(Deg::atan2(-1., 1.), Deg(-45.));
        assert_eq!(Grad::atan2(1., 0.), Grad(100.));
        assert!(close(Rad::atan2(0., -1.).0, PI));
    }

    #[test]
    fn test_inverse_round_trip() {
        for d in [-80., -30., 0., 10., 45., 89.] {
            assert!((Deg::asin(Deg(d).sin()).0 - d).abs() < 1e-4);
            assert!((Grad::atan(Grad(d).tan()).0 - d).abs() < 1e-4);
        }
    }

    #[test]
    fn test_per_turn() {
        assert_eq!(Rad::PER_TURN, TAU);
        assert_eq!(Deg::PER_TURN, 360.);
        assert_eq!(Grad::PER_TURN, 400.);
        assert_eq!(Tau::PER_TURN, 1.);
    }

    #[test]
    fn test_full_turn_converts_between_all_units() {
        assert_eq!(Deg::from(Rad(TAU)), Deg(360.));
        assert_eq!(Grad::from(Rad(TAU)), Grad(400.));
        assert_eq!(Rad::from(Deg(360.)), Rad(TAU));
        assert_eq!(Grad::from(Deg(360.)), Grad(400.));
        assert_eq!(Rad::from(Grad(400.)), Rad(TAU));
        assert_eq!(Deg::from(Grad(400.)), Deg(360.));
        assert_eq!(Tau::from(Rad(TAU)), Tau(1.));
        assert_eq!(Tau::from(Deg(360.)), Tau(1.));
        assert_eq!(Tau::from(Grad(400.)), Tau(1.));
        assert_eq!(Rad::from(Tau(1.)), Rad(TAU));
        assert_eq!(Deg::from(Tau(1.)), Deg(360.));
        assert_eq!(Grad::from(Tau(1.)), Grad(400.));
    }
}
