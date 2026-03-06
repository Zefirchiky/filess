use crate::{define_custom_quality_image, define_file, define_image_file};

pub struct GifConfig {
    /// 1-30, 1 - slowest lossless, 30 - fastest. 10 is good. Default: 1
    pub speed: i32
}

#[cfg(feature = "image")]
impl<'a> crate::primitives::ImageQualityConfig<'a> for GifConfig {
    type Encoder = image::codecs::gif::GifEncoder<&'a mut Vec<u8>>;
    fn get_encoder(&self, w: &'a mut Vec<u8>) -> Self::Encoder {
        image::codecs::gif::GifEncoder::new_with_speed(w, self.speed)
    }
}

define_file!(Gif, ["gif"]);
define_image_file!(Gif, image::ImageFormat::Gif);
define_custom_quality_image!(Gif, GifConfig);

#[cfg(test)]
mod gif_spec {
    // fn test() {
    //     image::codecs::tiff::TiffEncoder::
    // }
}