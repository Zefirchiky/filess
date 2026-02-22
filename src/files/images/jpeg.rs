use crate::{define_custom_quality_image, define_file, define_image_file};

#[cfg(feature = "image")]
#[derive(Debug, Clone, Copy)]
pub struct JpegConfig {
    /// 1-10, 1 - worst, 100 - best. Default: 75
    pub quality: u8,
}

#[cfg(feature = "image")]
impl<'a> crate::ImageQualityConfig<'a> for JpegConfig {
    type Encoder = image::codecs::jpeg::JpegEncoder<&'a mut Vec<u8>>;
    fn get_encoder(&self, w: &'a mut Vec<u8>) -> Self::Encoder {
        image::codecs::jpeg::JpegEncoder::new_with_quality(w, self.quality)
    }
}

define_file!(Jpeg, ["jpeg"], );
define_image_file!(Jpeg, image::ImageFormat::Jpeg);
define_custom_quality_image!(Jpeg, JpegConfig);

// #[cfg(test)]
// mod jpeg {
//     use crate::ImageQualityEncodingAsync;

//     use super::*;
    
//     #[test]
//     fn macros() {
//         Jpeg
//     }
// }
