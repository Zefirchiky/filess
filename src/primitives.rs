//! Re-exports of primitive types like [FileBase] and [FileCreationError].
#[cfg(feature = "audio")]
pub use crate::file_bases::audio_file::{DecodedStreamParams, DynamicDecoder};
pub use crate::file_bases::file_base::{FileBase, FileCreationError};
