//! Re-exports of error types for the crate.
pub use crate::file_bases::file_base::FileCreationError;
#[cfg(feature = "audio")]
pub use crate::file_bases::audio_file::AudioError;
#[cfg(feature = "image")]
pub use crate::file_bases::image_file::ImageIoError;
#[cfg(feature = "serde")]
pub use crate::file_bases::model_file::ModelIoError;
