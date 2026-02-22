#[cfg(feature = "jpeg")]
mod jpeg;
#[cfg(feature = "png")]
mod png;
#[cfg(feature = "webp")]
mod webp;
#[cfg(feature = "avif")]
mod avif;
#[cfg(feature = "tiff")]
mod tiff;

#[cfg(feature = "jpeg")]
pub use jpeg::Jpeg;
#[cfg(feature = "png")]
pub use png::Png;
#[cfg(feature = "webp")]
pub use webp::WebP;
#[cfg(feature = "avif")]
pub use avif::Avif;
#[cfg(feature = "tiff")]
pub use tiff::Tiff;
