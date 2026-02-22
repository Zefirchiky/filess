use derive_more::{AsRef, Deref, DerefMut, From};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{FileBase, FileTrait};
#[cfg(feature = "serde")]
use crate::{ModelFile};

#[derive(Debug, thiserror::Error)]
pub enum ModelTomlIoError {
    #[cfg(feature = "serde")]
    #[error("Seder Deserialization Error: {0}")]
    SerdeDeserialization(#[from] serde_toml::de::Error),
    #[cfg(feature = "serde")]
    #[error("Seder Serialization Error: {0}")]
    SerdeSerialization(#[from] serde_toml::ser::Error),
    #[error("Io Error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(feature = "serde")]
impl crate::ModelIoError for ModelTomlIoError {}

#[derive(Debug, Clone, Default, From, AsRef, Deref, DerefMut)]
#[from(forward)]
#[as_ref(forward)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Toml {
    file: FileBase<Self>,
}

impl Toml {
    /// Creates a new TomlHandler for the given file.
    ///
    /// If the file does not exist, it will be created. If the parent directories do not exist, they will be created.
    ///
    /// # Panics
    ///
    /// Panics if the path exists but is not a file, or if the file does not have the correct extension.
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        Self { file: FileBase::new(path) }
    }
}

impl FileTrait for Toml {
    fn ext() -> &'static [&'static str] {
        &["toml"]
    }
}

#[cfg(feature = "serde")]
impl ModelFile for Toml {
    type Error = ModelTomlIoError;

    fn bytes_to_model<T: for<'de> Deserialize<'de>>(data: Vec<u8>) -> Result<T, Self::Error> {
        Ok(serde_toml::from_slice(&data)?)
    }
    
    fn model_to_bytes(model: &impl Serialize) -> Result<Vec<u8>, Self::Error> {
        Ok(serde_toml::to_string_pretty(model)?.into())
    }
}
