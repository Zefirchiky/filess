use crate::{define_custom_quality_image, define_file, define_image_file};

#[cfg(feature = "image")]
pub struct AvifConfig {
    /// 1-10, 1 - slowest, 10 - fastest. Default: 4
    pub speed: u8,
    /// 1-10, 1 - worst, 100 - best. Default: 80
    pub quality: u8,
}

#[cfg(feature = "image")]
impl<'a> crate::ImageQualityConfig<'a> for AvifConfig {
    type Encoder = image::codecs::avif::AvifEncoder<&'a mut Vec<u8>>;
    fn get_encoder(&self, w: &'a mut Vec<u8>) -> Self::Encoder {
        image::codecs::avif::AvifEncoder::new_with_speed_quality(w, self.speed, self.quality)
    }
}

define_file!(Avif, ["avif"], );
define_image_file!(Avif, image::ImageFormat::Avif);
define_custom_quality_image!(Avif, AvifConfig);
