use derive_more::{AsRef, Deref, DerefMut, From};

#[cfg(feature = "image")]
use image::codecs::png::{CompressionType, FilterType};
use crate::{FileBase, FileTrait};

#[cfg(feature = "image")]
#[derive(Debug, Clone, Copy)]
pub struct PngConfig {
    compression: CompressionType,
    filter: FilterType,
}

#[cfg(feature = "image")]
impl PngConfig {
    pub fn new(compression: CompressionType, filter: FilterType) -> Self {
        Self { compression, filter }
    }
}

#[derive(Debug, Default, Clone, From, AsRef, Deref, DerefMut)]
#[from(forward)]
#[as_ref(forward)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Png {
    file: FileBase<Self>,
}

impl Png {
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        Self { file: FileBase::new(path) }
    }
}

impl FileTrait for Png {
    fn ext() -> &'static [&'static str] {
        &["png"]
    }
}

#[cfg(feature = "image")]
impl crate::ImageFile for Png {
    fn image_format() -> image::ImageFormat {
        image::ImageFormat::Png
    }
}

#[cfg(all(feature = "image", feature = "async"))]
impl crate::ImageFileAsync for Png {}

#[cfg(feature = "image")]
impl crate::ImageQulityEncoding for Png {
    type Config = PngConfig;
    
    fn get_encoder_w_quality(
        w: impl std::io::Write,
        Self::Config { compression, filter }: Self::Config,
    ) -> image::codecs::png::PngEncoder<impl std::io::Write> {
        image::codecs::png::PngEncoder::new_with_quality(w, compression, filter)
    }
}

#[cfg(all(feature = "image", feature = "async"))]
impl crate::ImageQualityEncodingAsync for Png {}
