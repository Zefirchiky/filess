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
    async fn asave_image(&self, img: &DynamicImage) -> Result<(), image::ImageError> {
        use std::io::{BufWriter, Cursor};

        use crate::file_base::FileTraitAsync;

        let mut buf = BufWriter::new(Cursor::new(vec![]));
        img.write_to(&mut buf, image::ImageFormat::from_path(&self)?)?;
        self.asave(&buf.buffer()).await?;
        Ok(())
    }

    async fn aload_image(&self) -> Result<DynamicImage, image::ImageError> {
        use std::io::{BufReader, Cursor};

        use crate::file_base::FileTraitAsync;

        Ok(ImageReader::new(BufReader::new(Cursor::new(self.aload().await?))).decode()?)
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
    /// Use `asave_image_custom` or `asave_image_custom_offload` if this is too slow and `async` feature is enabled.
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
    /// Use `asave_image_custom_offload` if this is too slow.
    async fn asave_image_custom(
        &self,
        img: &image::DynamicImage,
        config: Self::Config,
    ) -> Result<(), ImageIoError> {
        use crate::file_base::FileTraitAsync;

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
