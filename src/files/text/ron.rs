use crate::define_file;

#[cfg(feature = "serde")]
#[derive(Debug, thiserror::Error)]
/// Errors from RON model serialization/deserialization.
pub enum RonModelError {
    #[cfg(feature = "ron")]
    #[error("Seder Error: {0}")]
    SerdeDeserialization(#[from] ron::error::SpannedError),
    #[cfg(feature = "ron")]
    #[error("Seder Error: {0}")]
    SerdeSerialization(#[from] ron::Error),
    #[error("Io Error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(feature = "serde")]
impl crate::errors::ModelIoError for RonModelError {}

define_file!(
    Ron,
    "ron",
    ["application/ron", "text/ron"],
    ["ron"]
);

#[cfg(feature = "serde")]
impl crate::traits::ModelFile for Ron {
    type Error = RonModelError;

    fn bytes_to_model<T: for<'de> serde::Deserialize<'de>>(
        data: Vec<u8>,
    ) -> Result<T, Self::Error> {
        Ok(ron::de::from_bytes(&data)?)
    }

    fn model_to_bytes(model: &impl serde::Serialize) -> Result<Vec<u8>, Self::Error> {
        Ok(ron::ser::to_string_pretty(model, ron::ser::PrettyConfig::default())?.into())
    }
}
