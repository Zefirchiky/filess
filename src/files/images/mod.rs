#[cfg(feature = "jpeg")]
mod jpeg;
#[cfg(feature = "png")]
mod png;
#[cfg(feature = "webp")]
mod webp;

#[cfg(feature = "jpeg")]
pub use jpeg::Jpeg;
#[cfg(feature = "png")]
pub use png::Png;
#[cfg(feature = "webp")]
pub use webp::WebP;
