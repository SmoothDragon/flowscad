use std::ops::*;

use derive_more::Neg;

/// Millimeters
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Neg)]
pub struct Mm(pub f32);

/// Positive Mm MAX is lower since it is used for super large objects that could be shifted or rotated.
pub const MAX: Mm = Mm(f32::MAX / 1000.0);


// --- Multi-type Macro ---
macro_rules! impl_numeric_ops {
    ($($t:ty),*) => {
        $(
            // Primitive -> Mm
            impl From<$t> for Mm {
                fn from(x: $t) -> Mm {
                    Mm(x as f32)
                }
            }

            // Mm + Primitive -> Mm
            impl Add<$t> for Mm {
                type Output = Mm;
                #[inline]
                fn add(self, rhs: $t) -> Self::Output {
                    Mm(self.0 + rhs as f32)
                }
            }

            // Primitive + Mm -> Mm
            impl Add<Mm> for $t {
                type Output = Mm;
                #[inline]
                fn add(self, rhs: Mm) -> Self::Output {
                    Mm(self as f32 + rhs.0)
                }
            }

            // Mm += Primitive
            impl AddAssign<$t> for Mm {
                #[inline]
                fn add_assign(&mut self, rhs: $t) {
                    self.0 += rhs as f32;
                }
            }

            // Mm - Primitive -> Mm
            impl Sub<$t> for Mm {
                type Output = Mm;
                #[inline]
                fn sub(self, rhs: $t) -> Self::Output {
                    Mm(self.0 - rhs as f32)
                }
            }

            // Primitive - Mm -> Mm
            impl Sub<Mm> for $t {
                type Output = Mm;
                #[inline]
                fn sub(self, rhs: Mm) -> Self::Output {
                    Mm(self as f32 - rhs.0)
                }
            }

            // Mm -= Primitive
            impl SubAssign<$t> for Mm {
                #[inline]
                fn sub_assign(&mut self, rhs: $t) {
                    self.0 -= rhs as f32;
                }
            }

            // Mm * Primitive -> Mm
            impl Mul<$t> for Mm {
                type Output = Mm;
                #[inline]
                fn mul(self, rhs: $t) -> Self::Output {
                    Mm(self.0 * rhs as f32)
                }
            }

            // Primitive * Mm -> Mm
            impl Mul<Mm> for $t {
                type Output = Mm;
                #[inline]
                fn mul(self, rhs: Mm) -> Self::Output {
                    Mm(self as f32 * rhs.0)
                }
            }

            // Mm *= Primitive
            impl MulAssign<$t> for Mm {
                #[inline]
                fn mul_assign(&mut self, rhs: $t) {
                    self.0 *= rhs as f32;
                }
            }

            // Mm / Primitive -> Mm
            impl Div<$t> for Mm {
                type Output = Mm;
                #[inline]
                fn div(self, rhs: $t) -> Self::Output {
                    Mm(self.0 / rhs as f32)
                }
            }

            // Primitive / Mm -> Mm  -- Should not exist. Wrong unit.

            // Mm /= Primitive
            impl DivAssign<$t> for Mm {
                #[inline]
                fn div_assign(&mut self, rhs: $t) {
                    self.0 /= rhs as f32;
                }
            }
        )*
    };
}

// Apply the macro to all standard Rust numeric types
impl_numeric_ops!(f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

// --- Mm to Mm Operations ---
// Macros handle interactions with primitives, but you still need standard Mm-to-Mm operations

impl Add<Mm> for Mm {
    type Output = Mm;
    fn add(self, rhs: Mm) -> Self::Output {
        Mm(self.0 + rhs.0)
    }
}

impl AddAssign<Mm> for Mm {
    fn add_assign(&mut self, rhs: Mm) {
        self.0 += rhs.0;
    }
}


impl Sub<Mm> for Mm {
    type Output = Mm;
    fn sub(self, rhs: Mm) -> Self::Output {
        Mm(self.0 - rhs.0)
    }
}

impl SubAssign<Mm> for Mm {
    fn sub_assign(&mut self, rhs: Mm) {
        self.0 -= rhs.0;
    }
}

impl Div<Mm> for Mm {
    type Output = f32;
    fn div(self, rhs: Mm) -> Self::Output {
        self.0 / rhs.0
    }
}



impl Mm {
    pub fn powf<IX: Into<Mm>>(self, exp: IX) -> Self {
        Self(self.0.powf(exp.into().0))
    }

    pub fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }

    pub fn trunc(self) -> i32 {
        self.0 as i32
    }
}

/// `{}` prints the bare number for OpenSCAD output; `{:#}` appends the unit.
impl std::fmt::Display for Mm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)?;
        if f.alternate() {
            f.write_str("mm")?;
        }
        Ok(())
    }
}

// TODO: Fix ordering
// impl<IX: Into<Mm>> std::cmp::PartialOrd<IX> for Mm {
// type Output = bool;
// fn partial_cmp(&self, other: &IX) -> Option<std::cmp::Ordering> {
// Some(self.0.cmp(&other.into().0))
// }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_i32() {
        assert_eq!(Mm::from(5_i32), Mm(5.));
        assert_eq!(Mm::from(i32::MAX), Mm(2147483600.0));
    }

    #[test]
    fn test_into_real() {
        assert_eq!(<i32 as Into<Mm>>::into(5), Mm(5.));
        assert_eq!(<u32 as Into<Mm>>::into(5), Mm(5.));
        assert_eq!(<f64 as Into<Mm>>::into(5.0), Mm(5.));
    }

    #[test]
    fn test_real_mul() {
        assert_eq!(Mm(5.) * 2., Mm(10.));
        assert_eq!(Mm(5.) * 2, Mm(10.));
        assert_eq!(2. * Mm(5.), Mm(10.));
        assert_eq!(2 * Mm(5.), Mm(10.));
    }

    #[test]
    fn test_real_div() {
        assert_eq!(Mm(5.) / 2, Mm(2.5));
        assert_eq!(Mm(5.) / 0.5, Mm(10.));
    }

    #[test]
    fn test_real_add_sub() {
        assert_eq!(Mm(5.) + 2, Mm(7.));
        assert_eq!(2_u8 + Mm(5.), Mm(7.));
        assert_eq!(Mm(5.) - 2.5_f64, Mm(2.5));
        assert_eq!(10 - Mm(3.), Mm(7.));
    }

    #[test]
    fn test_mm_add_sub() {
        assert_eq!(Mm(3.) + Mm(4.), Mm(7.));
        assert_eq!(Mm(3.) - Mm(4.), Mm(-1.));
        assert_eq!(-Mm(3.), Mm(-3.));
    }

    #[test]
    fn test_mm_div() {
        assert_eq!(Mm(6.) / Mm(4.), 1.5_f32);
    }

    #[test]
    fn test_assign_ops() {
        let mut m = Mm(1.);
        m += 2;
        m -= 0.5_f64;
        m *= 4_u8;
        m /= 2_usize;
        m += Mm(1.);
        m -= Mm(0.5);
        assert_eq!(m, Mm(5.5));
    }

    #[test]
    fn test_ordering() {
        assert!(Mm(1.) < Mm(2.));
        assert!(Mm(-1.) <= Mm(-1.));
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Mm(2.5)), "2.5");
        assert_eq!(format!("{}", Mm(2.)), "2");
        assert_eq!(format!("{}", Mm(-2.5)), "-2.5");
        assert_eq!(Mm(2.5).to_string(), "2.5");
    }

    #[test]
    fn test_display_precision_and_width() {
        assert_eq!(format!("{:.2}", Mm(2.5)), "2.50");
        assert_eq!(format!("{:.0}", Mm(2.5)), "2");
        assert_eq!(format!("{:6}", Mm(2.5)), "   2.5");
        assert_eq!(format!("{:<6}|", Mm(2.5)), "2.5   |");
    }

    #[test]
    fn test_display_alternate() {
        assert_eq!(format!("{:#}", Mm(2.5)), "2.5mm");
        assert_eq!(format!("{:#}", Mm(2.)), "2mm");
        assert_eq!(format!("{:#}", Mm(-2.5)), "-2.5mm");
        assert_eq!(format!("{:#.1}", Mm(2.)), "2.0mm");
        assert_eq!(format!("{:#6}", Mm(2.5)), "   2.5mm");
    }

    #[test]
    fn test_display_in_scad_string() {
        assert_eq!(format!("sphere(r = {});", Mm(5.)), "sphere(r = 5);");
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", Mm(2.5)), "Mm(2.5)");
        assert_eq!(format!("{:?}", Mm(2.)), "Mm(2.0)");
        assert_eq!(format!("{:?}", Mm(-2.5)), "Mm(-2.5)");
        assert_eq!(format!("{:#?}", Mm(2.5)), "Mm(\n    2.5,\n)");
    }

    #[test]
    fn test_debug_nested() {
        assert_eq!(format!("{:?}", Some(Mm(1.))), "Some(Mm(1.0))");
        assert_eq!(format!("{:?}", vec![Mm(1.), Mm(2.5)]), "[Mm(1.0), Mm(2.5)]");
    }
}
