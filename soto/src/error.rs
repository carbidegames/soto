use std::error;
use std::fmt::{self, Display, Formatter};
use std::io;
use toml::de;

#[derive(Debug)]
pub enum Error {
    RequiredFileRead(String, Box<error::Error>),
    Io(io::Error),
    TomlParse(de::Error),
    Task(String),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::RequiredFileRead(_, _) => "Required File Read Error",
            Error::Io(_) => "IO Error",
            Error::TomlParse(_) => "Toml Parse Error",
            Error::Task(_) => "Task Running Error",
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Error::RequiredFileRead(ref file, ref e) =>
                write!(f, "Error while reading required file \"{}\": {}", file, e),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<de::Error> for Error {
    fn from(error: de::Error) -> Self {
        Error::TomlParse(error)
    }
}
