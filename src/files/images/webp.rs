use derive_more::{AsRef, Deref, DerefMut, From};

use crate::{FileBase, FileTrait};

#[derive(Debug, Default, Clone, From, AsRef, Deref, DerefMut)]
#[from(forward)]
#[as_ref(forward)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WebP {
    file: FileBase<Self>,
}

impl WebP {
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        Self { file: FileBase::new(path) }
    }
}

impl FileTrait for WebP {
    fn ext() -> &'static [&'static str] {
        &["webp"]
    }
}

#[cfg(feature = "image")]
impl crate::ImageFile for WebP {
    fn image_format() -> image::ImageFormat {
        image::ImageFormat::WebP
    }
}

#[cfg(all(feature = "image", feature = "async"))]
impl crate::ImageFileAsync for WebP {}
