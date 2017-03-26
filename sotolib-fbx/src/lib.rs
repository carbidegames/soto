extern crate fbx_direct;

pub mod animation;
pub mod simple;
mod raw;
mod tree;

pub use fbx_direct::reader::Error as FbxDirectError;
pub use fbx_direct::common::OwnedProperty;

pub use self::raw::{RawFbx, RawNode};
pub use self::tree::{ObjectTreeNode};

use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    FbxDirect(FbxDirectError),
    WrongNode(String, String), // Expected, Actual
    WrongNodeLayout(String),
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::FbxDirect(_) => "fbx_direct Parsing Error",
            Error::WrongNode(_, _) => "Wrong Node",
            Error::WrongNodeLayout(_) => "Wrong Node Layout",
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Error::WrongNode(ref expected, ref actual) =>
                write!(f, "Found {} instead of expected {}", actual, expected),
            _ => write!(f, "{:?}", self),
        }
    }
}

/// Converts unfriendly FBX names that contain \u{0} and \u{1} into friendly names.
pub fn friendly_name<T: AsRef<str>>(value: T) -> String {
    let parts: Vec<_> = value.as_ref().rsplit("\u{0}\u{1}").collect();
    parts.join("::")
}

/// Converts unfriendly FBX names that contain \u{0} and \u{1} into names that can be used as
/// identifiers.
pub fn id_name<T: AsRef<str>>(value: T) -> Option<String> {
    value.as_ref().split("\u{0}\u{1}").next().map(|s| s.to_string())
}
