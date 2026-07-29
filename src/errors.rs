//! Re-exports of error types for the crate.
#[cfg(feature = "audio")]
pub use crate::file_bases::audio_file::AudioError;
#[cfg(feature = "image")]
pub use crate::file_bases::image_file::ImageIoError;
#[cfg(feature = "_any_serde_model")]
pub use crate::file_bases::model_file::ModelIoError;
