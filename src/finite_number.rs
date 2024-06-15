#[derive(Debug, PartialEq)]
pub enum PositiveRealError {
    NonPositive,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Finite(pub f32);

impl Finite {
    /// Positive Real MAX is lower since it is used for super large objects that could be shifted or rotated.
    pub const MAX: Finite = Finite(f32::MAX/1000.0);
}

impl From<i32> for Finite {
    fn from(i: i32) -> Finite {
        Finite(i as f32)
    }
}

impl From<i64> for Finite {
    fn from(i: i64) -> Finite {
        Finite(i as f32)
    }
}

impl From<f32> for Finite {
    fn from(f: f32) -> Finite {
        Finite(f)
    }
}

impl From<f64> for Finite {
    fn from(f: f64) -> Finite {
        Finite(f as f32)
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
}
