use std::error::Error as StdError;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Error {
    ExampleError,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match self {
            ExampleError => write!(f, "ExampleError"),
        }
    }
}

impl StdError for Error {}
