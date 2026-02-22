use derive_more::{AsRef, Deref, DerefMut, From};

use crate::{FileBase, FileTrait};

#[cfg(feature = "image")]
#[derive(Debug, Clone, Copy)]
pub struct JpegConfig {
    quality: u8,
}

#[cfg(feature = "image")]
impl JpegConfig {
    pub fn new(quality: u8) -> Self {
        Self { quality }
    }
}

#[derive(Debug, Default, Clone, From, AsRef, Deref, DerefMut)]
#[from(forward)]
#[as_ref(forward)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Jpeg {
    file: FileBase<Self>,
}

impl Jpeg {
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        Self { file: FileBase::new(path) }
    }
}

impl FileTrait for Jpeg {
    fn ext() -> &'static [&'static str] {
        &["jpeg"]
    }
}

#[cfg(feature = "image")]
impl crate::ImageFile for Jpeg {
    fn image_format() -> image::ImageFormat {
        image::ImageFormat::Jpeg
    }
}

#[cfg(all(feature = "image", feature = "async"))]
impl crate::ImageFileAsync for Jpeg {}

#[cfg(feature = "image")]
impl crate::ImageQulityEncoding for Jpeg {
    type Config = JpegConfig;
    
    fn get_encoder_w_quality(
        w: impl std::io::Write,
        Self::Config { quality }: Self::Config,
    ) -> image::codecs::jpeg::JpegEncoder<impl std::io::Write> {
        image::codecs::jpeg::JpegEncoder::new_with_quality(w, quality)
    }
}

#[cfg(all(feature = "image", feature = "async"))]
impl crate::ImageQualityEncodingAsync for Jpeg {}
