use std::{path::Path};

use derive_more::{AsRef, Deref, DerefMut, From};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{FileBase, FileTrait};

#[derive(Debug, Clone, Default, From, AsRef, Deref, DerefMut)]
#[from(forward)]
#[as_ref(forward)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct File {
    file: FileBase,
}

impl File {
    pub fn new(file: impl AsRef<Path>) -> Self {
        Self::make_new(file)
    }
}

impl FileTrait for File {
    fn make_new(file: impl AsRef<Path>) -> Self {
        Self {
            file: FileBase::new_with_handler::<Self>(file),
        }
    }
    fn ext() -> &'static str {
        ""
    }
}

impl From<&str> for File {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}
