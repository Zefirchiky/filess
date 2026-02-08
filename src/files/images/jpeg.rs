use derive_more::{AsRef, Deref, DerefMut, From};

#[cfg(feature = "image")]
use crate::{ImageFileTrait, ImageFileEncodingTrait};
use crate::{FileBase, FileTrait};

#[derive(Debug, Default, Clone, From, AsRef, Deref, DerefMut)]
#[from(forward)]
#[as_ref(forward)]
pub struct Jpeg {
    file: FileBase,
}

impl Jpeg {
    pub fn new(file: impl AsRef<std::path::Path>) -> Self {
        Self::make_new(file)
    }
}

impl FileTrait for Jpeg {
    fn ext() -> &'static [&'static str] {
        &["jpeg"]
    }

    fn make_new(file: impl AsRef<std::path::Path>) -> Self {
        Self {
            file: FileBase::new_with_handler::<Self>(file),
        }
    }
}

#[cfg(feature = "image")]
impl ImageFileTrait for Jpeg {}

#[cfg(feature = "image")]
impl ImageFileEncodingTrait for Jpeg {
    fn get_encoder_w_quality(
        w: impl std::io::Write,
        quality: u8,
    ) -> image::codecs::jpeg::JpegEncoder<impl std::io::Write> {
        image::codecs::jpeg::JpegEncoder::new_with_quality(w, quality)
    }
}

impl From<&str> for Jpeg {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}
