use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum SotoError {
    RequiredFileReadError(String, Box<Error>)
}

impl Error for SotoError {
    fn description(&self) -> &str {
        match *self {
            SotoError::RequiredFileReadError(_, _) => "Required File Read Error",
        }
    }
}

impl Display for SotoError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            SotoError::RequiredFileReadError(ref file, ref e) =>
                write!(f, "Error while reading required file \"{}\": {}", file, e),
            //_ => write!(f, "{:?}", self)
        }
    }
}
