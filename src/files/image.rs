use std::path::Path;

use derive_more::{AsRef, Deref, DerefMut, From};
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use crate::{FileBase, FileTrait};

#[derive(Debug, Default, Clone, From, AsRef, Deref, DerefMut)]
#[as_ref(forward)]
#[from(forward)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Image {
    file: FileBase,
}

impl Image {
    pub fn new(file: impl AsRef<Path>) -> Self {
        Self::make_new(file)
    }
}

impl FileTrait for Image {
    fn ext() -> &'static str {
        ""
    }

    fn initialize_file(_file: &mut std::fs::File) {}

    fn make_new(file: impl AsRef<std::path::Path>) -> Self {
        Self {
            file: FileBase::new_with_handler::<Self>(file)
        }
    }
}

impl From<&str> for Image {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}