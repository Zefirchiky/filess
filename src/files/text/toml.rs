use crate::define_file;

#[cfg(feature = "serde")]
#[derive(Debug, thiserror::Error)]
/// Errors from TOML model serialization/deserialization.
pub enum TomlModelError {
    #[cfg(feature = "toml")]
    #[error("Seder Deserialization Error: {0}")]
    SerdeDeserialization(#[from] serde_toml::de::Error),
    #[cfg(feature = "toml")]
    #[error("Seder Serialization Error: {0}")]
    SerdeSerialization(#[from] serde_toml::ser::Error),
    #[error("Io Error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(feature = "serde")]
impl crate::errors::ModelIoError for TomlModelError {}

define_file!(
    Toml,
    "toml",
    ["application/toml", "text/x-toml", "text/ini"],
    ["toml"]
);

#[cfg(feature = "serde")]
impl crate::traits::ModelFile for Toml {
    type Error = TomlModelError;

    fn bytes_to_model<T: for<'de> serde::Deserialize<'de>>(
        data: Vec<u8>,
    ) -> Result<T, Self::Error> {
        Ok(serde_toml::from_slice(&data)?)
    }

    fn model_to_bytes(model: &impl serde::Serialize) -> Result<Vec<u8>, Self::Error> {
        Ok(serde_toml::to_string_pretty(model)?.into())
    }
}
