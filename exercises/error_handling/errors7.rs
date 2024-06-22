// errors7.rs

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct PosNonZeroInteger(u64);

#[derive(Debug, PartialEq)]
enum CreationError {
    Zero,
    Negative,
}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CreationError::Negative => write!(f, "Negative results are not allowed"),
            CreationError::Zero => write!(f, "Zero value is not allowed"),
        }
    }
}

impl Error for CreationError {}

impl PosNonZeroInteger {
    fn new(&self, value: i64) -> Result<i64, CreationError> {
        match value {
            v if v == 0 => Err(CreationError::Zero),
            v if v < 0 => Err(CreationError::Negative),
            v => Ok(v)
        }
    }
}

#[derive(Debug, PartialEq)]
enum PosNonZeroIntegerError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl PosNonZeroIntegerError {
    fn from_creation(e: CreationError) -> PosNonZeroIntegerError { 
        PosNonZeroIntegerError::Creation(e)
    }

    fn from_parseint(e: ParseIntError) -> PosNonZeroIntegerError {
        PosNonZeroIntegerError::ParseInt(e)
    }
}

impl fmt::Display for PosNonZeroIntegerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PosNonZeroIntegerError::Creation(e) => write!(f, "Creation error: {}", e),
            PosNonZeroIntegerError::ParseInt(e) => write!(f, "ParseInt error: {}", e),
        }
    }
}

impl Error for PosNonZeroIntegerError {}



fn parse_pos_nonzero(value: &str) -> Result<PosNonZeroInteger, PosNonZeroIntegerError> {
    let integer: i64 = match value.parse() {
        Ok(v) => v,
        Err(e) => return Err(PosNonZeroIntegerError::from_parseint(e))
    };

    match integer {
        v if v == 0 => Err(PosNonZeroIntegerError::from_creation(CreationError::Zero)),
        v if v < 0 => Err(PosNonZeroIntegerError::from_creation(CreationError::Negative)),
        v => Ok(PosNonZeroInteger(v as u64))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_posnonzerointeger() {
        let mock_user_input = "42";
        assert_eq!(
            parse_pos_nonzero(mock_user_input),
            Ok(PosNonZeroInteger(42 as u64))
        )
    }

    #[test]
    fn test_negative_integer() {
        let mock_user_input = "-100";
        assert!(matches!(
            parse_pos_nonzero(mock_user_input),
            Err(PosNonZeroIntegerError::Creation(CreationError::Negative))
        ))
    }

    #[test]
    fn test_zero_integer() {
        let mock_user_input = "0";
        assert!(matches!(
            parse_pos_nonzero(mock_user_input),
            Err(PosNonZeroIntegerError::Creation(CreationError::Zero))
        ))
    }

    #[test]
    fn test_parse_error() {
        let mock_user_input = "not an integer";
        assert!(matches!(
            parse_pos_nonzero(mock_user_input),
            Err(PosNonZeroIntegerError::ParseInt(_))
        ))
    }
}
