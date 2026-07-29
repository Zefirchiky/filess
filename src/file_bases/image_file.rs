use std::fmt::Debug;

use image::{DynamicImage, ImageReader};

use crate::traits::FileTrait;

#[derive(Debug, thiserror::Error)]
/// Errors from image I/O operations.
pub enum ImageIoError {
    #[error("Image Error: {0}")]
    Image(#[from] image::ImageError),
    #[error("Io Error: {0}")]
    Io(#[from] std::io::Error),
}

/// Trait for image files that can load/save [DynamicImage].
pub trait ImageFile: FileTrait {
    /// Saves [DynamicImage] with default settings
    fn save_image(&self, img: &DynamicImage) -> Result<(), image::ImageError> {
        img.save(&self)
    }

    /// Loads and decodes an image from the file.
    fn load_image(&self) -> Result<DynamicImage, image::ImageError> {
        Ok(ImageReader::open(&self)?.decode()?)
    }

    /// Returns the image format for encoding.
    fn image_format() -> image::ImageFormat;
}

#[cfg(feature = "async")]
/// Async counterpart of [ImageFile].
pub trait AsyncImageFile: ImageFile + crate::traits::AsyncFileTrait {
    /// Async version of [ImageFile::save_image].
    async fn asave_image(&self, img: &DynamicImage) -> Result<(), image::ImageError> {
        use std::io::{BufWriter, Cursor};

        let mut buf = BufWriter::new(Cursor::new(vec![]));
        img.write_to(&mut buf, image::ImageFormat::from_path(&self)?)?;
        self.asave(&buf.buffer()).await?;
        Ok(())
    }

    /// Async version of [ImageFile::load_image].
    async fn aload_image(&self) -> Result<DynamicImage, image::ImageError> {
        use std::io::{BufReader, Cursor};

        Ok(ImageReader::new(BufReader::new(Cursor::new(self.aload().await?))).decode()?)
    }
}

#[cfg(feature = "async")]
impl<T: ImageFile + crate::traits::AsyncFileTrait> AsyncImageFile for T {}

/// Configuration for a quality-tunable image encoder.
pub trait ImageQualityConfig<'a>: Debug + Default + Clone + Copy {
    type Encoder: image::ImageEncoder;
    /// Returns an encoder configured with the quality settings.
    fn get_encoder(&self, w: &'a mut Vec<u8>) -> Self::Encoder;
}

/// Trait for file types supporting custom-quality image encoding.
pub trait ImageQualityEncoding: FileTrait {
    type Config: for<'a> ImageQualityConfig<'a> + Sync + Send;

    /// Save image with custom quality.
    ///
    /// Use [asave_image_custom](AsyncImageQualityEncoding::asave_image_custom) or
    /// [asave_image_custom_offload](AsyncImageQualityEncoding::asave_image_custom_offload)
    /// if this is too slow and `async` feature is enabled.
    fn save_image_custom(
        &self,
        img: &image::DynamicImage,
        config: Self::Config,
    ) -> Result<(), ImageIoError> {
        let mut buf = vec![];
        img.write_with_encoder(config.get_encoder(&mut buf))?;
        self.save(&buf)?;
        Ok(())
    }
}

#[cfg(feature = "async")]
/// Async counterpart of [ImageQualityEncoding].
pub trait AsyncImageQualityEncoding: ImageQualityEncoding + crate::traits::AsyncFileTrait {
    /// Save image with custom quality.
    ///
    /// Use [asave_image_custom_offload](AsyncImageQualityEncoding::asave_image_custom_offload) if this is too slow.
    async fn asave_image_custom(
        &self,
        img: &image::DynamicImage,
        config: Self::Config,
    ) -> Result<(), ImageIoError> {
        let mut buf = vec![];
        img.write_with_encoder(config.get_encoder(&mut buf))?;
        self.asave(&buf).await?;
        Ok(())
    }

    /// Save image with `offload` function and custom quality.
    ///
    /// Use if encoding image is expensive and you want to offload it into a separate thread/async.
    async fn asave_image_custom_offload<'a, F>(
        &'a self,
        img: &'a image::DynamicImage,
        config: Self::Config,
        offload: F,
    ) -> Result<(), ImageIoError>
    where
        F: FnOnce(Box<dyn FnOnce() -> Result<(), ImageIoError> + Send + 'a>),
        F::Output: Future<Output = Result<(), ImageIoError>>,
        Self: Sync + Send,
    {
        (offload)(Box::new(move || self.save_image_custom(&img, config))).await
    }
}

#[cfg(feature = "async")]
impl<T: ImageQualityEncoding + crate::traits::AsyncFileTrait> AsyncImageQualityEncoding for T {}