extern crate fbx_direct;

mod raw;
mod simple;

pub use fbx_direct::reader::Error as FbxDirectError;

pub use self::raw::{RawFbx, FbxNode};
pub use self::simple::{SimpleFbx, FbxModel, FbxGeometry, FbxConnection, FbxObject};

use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum FbxError {
    FbxDirect(FbxDirectError)
}

impl Error for FbxError {
    fn description(&self) -> &str {
        match *self {
            FbxError::FbxDirect(_) => "fbx_direct Parsing Error",
        }
    }
}

impl Display for FbxError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Converts unfriendly FBX names that contain \u{0} and \u{1} into friendly names.
pub fn friendly_name<T: AsRef<str>>(value: T) -> String {
    let parts: Vec<_> = value.as_ref().rsplit("\u{0}\u{1}").collect();
    parts.join("::")
}
