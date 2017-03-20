use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;
use toml::de;

#[derive(Debug)]
pub enum SotoError {
    RequiredFileReadError(String, Box<Error>),
    IoError(io::Error),
    TomlParseError(de::Error),
}

impl Error for SotoError {
    fn description(&self) -> &str {
        match *self {
            SotoError::RequiredFileReadError(_, _) => "Required File Read Error",
            SotoError::IoError(_) => "IO Error",
            SotoError::TomlParseError(_) => "Toml Parse Error",
        }
    }
}

impl Display for SotoError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            SotoError::RequiredFileReadError(ref file, ref e) =>
                write!(f, "Error while reading required file \"{}\": {}", file, e),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl From<io::Error> for SotoError {
    fn from(error: io::Error) -> Self {
        SotoError::IoError(error)
    }
}

impl From<de::Error> for SotoError {
    fn from(error: de::Error) -> Self {
        SotoError::TomlParseError(error)
    }
}
