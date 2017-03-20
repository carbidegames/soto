use std::error;
use std::fmt::{self, Display, Formatter};
use std::io;
use toml::de;

#[derive(Debug)]
pub enum Error {
    RequiredFileReadError(String, Box<error::Error>),
    IoError(io::Error),
    TomlParseError(de::Error),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::RequiredFileReadError(_, _) => "Required File Read Error",
            Error::IoError(_) => "IO Error",
            Error::TomlParseError(_) => "Toml Parse Error",
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Error::RequiredFileReadError(ref file, ref e) =>
                write!(f, "Error while reading required file \"{}\": {}", file, e),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<de::Error> for Error {
    fn from(error: de::Error) -> Self {
        Error::TomlParseError(error)
    }
}
