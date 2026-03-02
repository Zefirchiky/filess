use image::{DynamicImage, ImageReader};

use crate::FileTrait;

#[derive(Debug, thiserror::Error)]
pub enum ImageIoError {
    #[error("Image Error: {0}")]
    Image(#[from] image::ImageError),
    #[error("Io Error: {0}")]
    Io(#[from] std::io::Error),
}

pub trait ImageFile: FileTrait {
    fn save_image(&self, img: &DynamicImage) -> Result<(), image::ImageError> {
        img.save(&self)
    }

    fn load_image(&self) -> Result<DynamicImage, image::ImageError> {
        Ok(ImageReader::open(&self)?.decode()?)
    }

    fn image_format() -> image::ImageFormat;
}

#[cfg(feature = "async")]
pub trait ImageFileAsync: ImageFile {
    async fn save_image_async(&self, img: &DynamicImage) -> Result<(), image::ImageError> {
        use std::io::{BufWriter, Cursor};

        let mut buf = BufWriter::new(Cursor::new(vec![]));
        img.write_to(&mut buf, image::ImageFormat::from_path(&self)?)?;
        self.save_async(&buf.buffer()).await?;
        Ok(())
    }

    async fn load_image_async(&self) -> Result<DynamicImage, image::ImageError> {
        use std::io::{BufReader, Cursor};

        Ok(ImageReader::new(BufReader::new(Cursor::new(self.load_async().await?))).decode()?)
    }
}

pub trait ImageQualityConfig<'a> {
    type Encoder: image::ImageEncoder;
    fn get_encoder(&self, w: &'a mut Vec<u8>) -> Self::Encoder;
}

pub trait ImageQulityEncoding: FileTrait {
    type Config: for<'a> ImageQualityConfig<'a> + Sync + Send;

    /// Save image with custom quality.
    ///
    /// Use `save_image_custom_async` or `save_image_custom_async_offload` if this is too slow and `async` feature is enabled.
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
pub trait ImageQualityEncodingAsync: ImageQulityEncoding {
    /// Save image with custom quality.
    ///
    /// Use `save_image_custom_async_offload` if this is too slow.
    async fn save_image_custom_async(
        &self,
        img: &image::DynamicImage,
        config: Self::Config,
    ) -> Result<(), ImageIoError> {
        let mut buf = vec![];
        img.write_with_encoder(config.get_encoder(&mut buf))?;
        self.save_async(&buf).await?;
        Ok(())
    }

    /// Save image with `offload` function and custom quality.
    ///
    /// Use if encoding image is expensive and you want to offload it into a separate thread/async.
    async fn save_image_custom_async_offload<'a, F>(
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
