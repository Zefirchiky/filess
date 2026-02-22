#[cfg(feature = "image")]
use image::codecs::png::{CompressionType, FilterType};
use crate::{define_custom_quality_image, define_file, define_image_file};

#[cfg(feature = "image")]
#[derive(Debug, Clone, Copy)]
pub struct PngConfig {
    pub compression: CompressionType,
    pub filter: FilterType,
}

#[cfg(feature = "image")]
impl<'a> crate::ImageQualityConfig<'a> for PngConfig {
    type Encoder = image::codecs::png::PngEncoder<&'a mut Vec<u8>>;
    fn get_encoder(&self, w: &'a mut Vec<u8>) -> Self::Encoder {
        image::codecs::png::PngEncoder::new_with_quality(w, self.compression, self.filter)
    }
}

define_file!(Png, ["png"], );
define_image_file!(Png, image::ImageFormat::Png);
define_custom_quality_image!(Png, PngConfig);
