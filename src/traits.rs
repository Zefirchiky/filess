#[cfg(feature = "audio")]
pub use crate::file_bases::audio_file::{AudioCodecsFile, AudioContainerFile, AudioFile};
pub use crate::file_bases::file_base::FileTrait;
pub use crate::file_bases::fs_element::*;
#[cfg(feature = "image")]
pub use crate::file_bases::image_file::{ImageFile, ImageQualityConfig, ImageQualityEncoding};
#[cfg(feature = "serde")]
pub use crate::file_bases::model_file::ModelFile;

#[cfg(feature = "open")]
pub use crate::open_integration::OpenTrait;

#[cfg(feature = "async")]
pub use _async::*;

#[cfg(feature = "async")]
mod _async {
    pub use crate::file_bases::file_base::AsyncFileTrait;
    #[cfg(feature = "image")]
    pub use crate::file_bases::image_file::{AsyncImageFile, AsyncImageQualityEncoding};
    #[cfg(feature = "serde")]
    pub use crate::file_bases::model_file::AsyncModelFile;
    // #[cfg(feature = "audio")]    // TODO: Add async audio
    // pub use crate::file_bases::audio_file::;
}
