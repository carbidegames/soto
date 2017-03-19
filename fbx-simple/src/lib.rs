extern crate fbx_direct;

mod raw;
mod simple;

pub use fbx_direct::reader::Error as FbxDirectError;

pub use self::raw::{RawFbx, FbxNode};
pub use self::simple::{SimpleFbx, FbxModel, FbxConnection};

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
