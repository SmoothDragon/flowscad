#[derive(Debug, PartialEq)]
pub enum PositiveRealError {
    NonPositive,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PositiveReal(pub f32);

impl PositiveReal {
    pub const MAX: PositiveReal = PositiveReal(f32::MAX);
}

impl TryFrom<i32> for PositiveReal {
    type Error = PositiveRealError;

    fn try_from(i: i32) -> Result<PositiveReal, PositiveRealError> {
        if i > 0 {
            Ok(PositiveReal(i as f32))
        } else {
            Err(PositiveRealError::NonPositive)
        }
    }
}

impl TryFrom<i64> for PositiveReal {
    type Error = PositiveRealError;

    fn try_from(i: i64) -> Result<PositiveReal, PositiveRealError> {
        if i > 0 {
            Ok(PositiveReal(i as f32))
        } else {
            Err(PositiveRealError::NonPositive)
        }
    }
}

impl TryFrom<f32> for PositiveReal {
    type Error = PositiveRealError;

    fn try_from(f: f32) -> Result<PositiveReal, PositiveRealError> {
        if f > 0. && f < f32::INFINITY {
            Ok(PositiveReal(f))
        } else {
            Err(PositiveRealError::NonPositive)
        }
    }
}

impl TryFrom<f64> for PositiveReal {
    type Error = PositiveRealError;

    fn try_from(f: f64) -> Result<PositiveReal, PositiveRealError> {
        if f > 0. && f < f64::INFINITY {
            Ok(PositiveReal(f as f32))
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
        assert_eq!(PositiveReal::try_from(5_i32), Ok(PositiveReal(5.)));
        assert_eq!(PositiveReal::try_from(0), Err(PositiveRealError::NonPositive));
        assert_eq!(PositiveReal::try_from(-5_i32), Err(PositiveRealError::NonPositive));
        assert_eq!(PositiveReal::try_from(i32::MAX), Ok(PositiveReal(2147483600.0)));
    }

    #[test]
    fn test_try_from_i64() {
        assert_eq!(PositiveReal::try_from(5_i64), Ok(PositiveReal(5.)));
        assert_eq!(PositiveReal::try_from(i64::MAX), Ok(PositiveReal(9.223372036854776e18)));
    }

    #[test]
    fn test_try_from_f32() {
        assert_eq!(PositiveReal::try_from(5_f32), Ok(PositiveReal(5.)));
        assert_eq!(PositiveReal::try_from(-5_f32), Err(PositiveRealError::NonPositive));
        assert_eq!(PositiveReal::try_from(f32::NAN), Err(PositiveRealError::NonPositive));
        assert_eq!(PositiveReal::try_from(f32::INFINITY), Err(PositiveRealError::NonPositive));
        assert_eq!(PositiveReal::try_from(f32::NEG_INFINITY), Err(PositiveRealError::NonPositive));
    }

    #[test]
    fn test_try_from_f64() {
        assert_eq!(PositiveReal::try_from(5_f64), Ok(PositiveReal(5.)));
        assert_eq!(PositiveReal::try_from(f64::NAN), Err(PositiveRealError::NonPositive));
        assert_eq!(PositiveReal::try_from(f64::INFINITY), Err(PositiveRealError::NonPositive));
        assert_eq!(PositiveReal::try_from(f64::NEG_INFINITY), Err(PositiveRealError::NonPositive));
    }
}
