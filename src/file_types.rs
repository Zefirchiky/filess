use crate::{File, Image};

#[derive(Debug)]
pub enum FileType {
    File(File),
    Image(Image),
    #[cfg(feature = "json")]
    Json(crate::Json),
    #[cfg(feature = "toml")]
    Toml(crate::Toml),
    #[cfg(feature = "md")]
    Md(crate::Md),
    #[cfg(feature = "jpeg")]
    Jpeg(crate::Jpeg),
    #[cfg(feature = "png")]
    Png(crate::Png),
    #[cfg(feature = "webp")]
    WebP(crate::WebP),
}

pub enum TextTypes {
    File(File),
    #[cfg(feature = "json")]
    Json(crate::Json),
    #[cfg(feature = "toml")]
    Toml(crate::Toml),
    #[cfg(feature = "md")]
    Md(crate::Md),
}

#[cfg(feature = "serde")]
pub enum ModelTypes {
    #[cfg(feature = "json")]
    Json(crate::Json),
    #[cfg(feature = "toml")]
    Toml(crate::Toml),
}

#[cfg(feature = "image")]
pub enum ImageType {
    Image(Image),
    #[cfg(feature = "jpeg")]
    Jpeg(crate::Jpeg),
    #[cfg(feature = "png")]
    Png(crate::Png),
    #[cfg(feature = "webp")]
    WebP(crate::WebP),
}
