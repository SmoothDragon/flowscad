use std::ops::*;
use std::fmt;
use derive_more::*;

#[derive(Debug, PartialEq)]
pub enum PositiveRealError {
    NonPositive,
}

#[derive(Clone, Copy, PartialEq, Add, Mul, Neg)]
pub struct Real(pub f32);

impl Real {
    /// Positive Real MAX is lower since it is used for super large objects that could be shifted or rotated.
    pub const MAX: Real = Real(f32::MAX/1000.0);
}

impl fmt::Debug for Real {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl fmt::Display for Real {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl AddAssign for Real {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0);
    }
}

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
        Real(f)
    }
}

impl From<f64> for Real {
    fn from(f: f64) -> Real {
        Real(f as f32)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FinitePositive(pub f32);

impl FinitePositive {
    /// Positive Real MAX is lower since it is used for super large objects that could be shifted or rotated.
    pub const MAX: FinitePositive = FinitePositive(f32::MAX/1000.0);
}

impl TryFrom<i32> for FinitePositive {
    type Error = PositiveRealError;

    fn try_from(i: i32) -> Result<FinitePositive, PositiveRealError> {
        if i > 0 {
            Ok(FinitePositive(i as f32))
        } else {
            Err(PositiveRealError::NonPositive)
        }
    }
}

impl TryFrom<i64> for FinitePositive {
    type Error = PositiveRealError;

    fn try_from(i: i64) -> Result<FinitePositive, PositiveRealError> {
        if i > 0 {
            Ok(FinitePositive(i as f32))
        } else {
            Err(PositiveRealError::NonPositive)
        }
    }
}

impl TryFrom<f32> for FinitePositive {
    type Error = PositiveRealError;

    fn try_from(f: f32) -> Result<FinitePositive, PositiveRealError> {
        if f > 0. && f < f32::INFINITY {
            Ok(FinitePositive(f))
        } else {
            Err(PositiveRealError::NonPositive)
        }
    }
}

impl TryFrom<f64> for FinitePositive {
    type Error = PositiveRealError;

    fn try_from(f: f64) -> Result<FinitePositive, PositiveRealError> {
        if f > 0. && f < f64::INFINITY {
            Ok(FinitePositive(f as f32))
        } else {
            Err(PositiveRealError::NonPositive)
        }
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
    fn test_try_from_i32() {
        assert_eq!(FinitePositive::try_from(5_i32), Ok(FinitePositive(5.)));
        assert_eq!(FinitePositive::try_from(0), Err(PositiveRealError::NonPositive));
        assert_eq!(FinitePositive::try_from(-5_i32), Err(PositiveRealError::NonPositive));
        assert_eq!(FinitePositive::try_from(i32::MAX), Ok(FinitePositive(2147483600.0)));
    }

    #[test]
    fn test_try_from_i64() {
        assert_eq!(FinitePositive::try_from(5_i64), Ok(FinitePositive(5.)));
        assert_eq!(FinitePositive::try_from(i64::MAX), Ok(FinitePositive(9.223372036854776e18)));
    }

    #[test]
    fn test_try_from_f32() {
        assert_eq!(FinitePositive::try_from(5_f32), Ok(FinitePositive(5.)));
        assert_eq!(FinitePositive::try_from(-5_f32), Err(PositiveRealError::NonPositive));
        assert_eq!(FinitePositive::try_from(f32::NAN), Err(PositiveRealError::NonPositive));
        assert_eq!(FinitePositive::try_from(f32::INFINITY), Err(PositiveRealError::NonPositive));
        assert_eq!(FinitePositive::try_from(f32::NEG_INFINITY), Err(PositiveRealError::NonPositive));
    }

    #[test]
    fn test_try_from_f64() {
        assert_eq!(FinitePositive::try_from(5_f64), Ok(FinitePositive(5.)));
        assert_eq!(FinitePositive::try_from(f64::NAN), Err(PositiveRealError::NonPositive));
        assert_eq!(FinitePositive::try_from(f64::INFINITY), Err(PositiveRealError::NonPositive));
        assert_eq!(FinitePositive::try_from(f64::NEG_INFINITY), Err(PositiveRealError::NonPositive));
    }

    #[test]
    fn test_mul() {
        assert_eq!(FinitePositive::try_from(5_f64), Ok(FinitePositive(5.)));
        assert_eq!(FinitePositive::try_from(f64::NAN), Err(PositiveRealError::NonPositive));
        assert_eq!(FinitePositive::try_from(f64::INFINITY), Err(PositiveRealError::NonPositive));
        assert_eq!(FinitePositive::try_from(f64::NEG_INFINITY), Err(PositiveRealError::NonPositive));
    }
}
