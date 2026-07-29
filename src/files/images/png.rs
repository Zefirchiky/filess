use crate::{define_custom_quality_image, define_file, define_image_file};
#[cfg(feature = "image")]
use image::codecs::png::{CompressionType, FilterType};

#[cfg(feature = "image")]
#[derive(Debug, Default, Clone, Copy)]
/// Compression and filter configuration for PNG encoding.
pub struct PngConfig {
    pub compression: CompressionType,
    pub filter: FilterType,
}

#[cfg(feature = "image")]
impl<'a> crate::traits::ImageQualityConfig<'a> for PngConfig {
    type Encoder = image::codecs::png::PngEncoder<&'a mut Vec<u8>>;
    fn get_encoder(&self, w: &'a mut Vec<u8>) -> Self::Encoder {
        image::codecs::png::PngEncoder::new_with_quality(w, self.compression, self.filter)
    }
}

define_file!(
    Png,
    "png",
    [
        "image/png",
        "image/vnd.mozilla.apng",
        "application/octet-stream",
        "image/apng"
    ],
    ["png"]
);
define_image_file!(Png, image::ImageFormat::Png);
define_custom_quality_image!(Png, PngConfig);

#[cfg(all(test, feature = "image"))]
mod png_tests {
    use std::env::temp_dir;

    use crate::{Temporary, traits::{ImageFile, ImageQualityEncoding}};

    use super::*;

    #[test]
    fn save_load_image() {
        let dir = temp_dir();
        let p = dir.join("png_img.png");
        let f = Temporary::new(Png::new(&p));
        let img = image::DynamicImage::from(image::RgbaImage::new(4, 4));
        f.save_image(&img).unwrap();
        let loaded = f.load_image().unwrap();
        assert_eq!(loaded.width(), 4);
        assert_eq!(loaded.height(), 4);
    }

    #[test]
    fn save_image_custom() {
        use image::codecs::png::{CompressionType, FilterType};
        let dir = temp_dir();
        let p = dir.join("png_custom.png");
        let f = Temporary::new(Png::new(&p));
        let cfg = PngConfig {
            compression: CompressionType::Default,
            filter: FilterType::Sub,
        };
        let img = image::DynamicImage::from(image::RgbaImage::new(2, 2));
        f.save_image_custom(&img, cfg).unwrap();
        assert!(p.exists());
        let loaded = image::load_from_memory(&std::fs::read(&p).unwrap()).unwrap();
        assert_eq!(loaded.width(), 2);
    }
}
