use std::{fs::File, io::Write, path::Path};

use derive_more::{AsRef, Deref, DerefMut, From};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{FileBase, FileTrait};
#[cfg(feature = "serde")]
use crate::{ModelFileTrait, model_file::ModelIoError};

#[derive(Debug, thiserror::Error)]
pub enum ModelTomlIoError {
    #[cfg(feature = "serde")]
    #[error("Seder Error: {0}")]
    Serde(#[from] serde_toml::Error),
    #[error("Io Error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(feature = "serde")]
impl ModelIoError for ModelTomlIoError {}

#[derive(Debug, Clone, Default, From, AsRef, Deref, DerefMut)]
#[from(forward)]
#[as_ref(forward)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Toml {
    handler: FileBase,
}

impl Toml {
    /// Creates a new TomlHandler for the given file.
    ///
    /// If the file does not exist, it will be created. If the parent directories do not exist, they will be created.
    ///
    /// # Panics
    ///
    /// Panics if the path exists but is not a file, or if the file does not have the correct extension.
    pub fn new(file: impl AsRef<Path>) -> Self {
        Self::make_new(file)
    }
}

impl FileTrait for Toml {
    fn make_new(file: impl AsRef<Path>) -> Self {
        Self {
            handler: FileBase::new_with_handler::<Self>(file),
        }
    }

    fn initialize_file(file: &mut File) {
        file.write_all(b"{}")
            .expect("Failed to write initial Toml content");
    }

    fn ext() -> &'static str {
        "toml"
    }
}

#[cfg(feature = "serde")]
impl ModelFileTrait for Toml {
    type Error = ModelTomlIoError;

    fn load_model<T: DeserializeOwned>(&self) -> Result<T, Self::Error> {
        let data = self.load()?;
        Ok(serde_toml::from_str(&data)?)
    }

    fn save_model(&self, model: &impl Serialize) -> Result<(), Self::Error> {
        self.save(serde_toml::to_string_pretty(model)?)?;
        Ok(())
    }
}

impl From<&str> for Toml {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}