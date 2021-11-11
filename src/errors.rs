use std::fmt;
use std::io;

#[derive(Debug)]
pub struct ReadError {
    pub message: String,
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<&str> for ReadError {
    fn from(s: &str) -> Self {
        ReadError {
            message: s.to_string(),
        }
    }
}

impl From<io::Error> for ReadError {
    fn from(e: io::Error) -> Self {
        ReadError {
            message: e.to_string(),
        }
    }
}

impl From<std::num::TryFromIntError> for ReadError {
    fn from(_e: std::num::TryFromIntError) -> Self {
        ReadError {
            message: "Number conversion error".to_string(),
        }
    }
}

