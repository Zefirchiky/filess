use crate::{define_custom_quality_image, define_file, define_image_file};

#[cfg(feature = "image")]
#[derive(Debug, Clone, Copy)]
/// Quality configuration for JPEG encoding.
pub struct JpegConfig {
    /// 1-10, 1 - worst, 100 - best. Default: 75
    pub quality: u8,
}

#[cfg(feature = "image")]
impl Default for JpegConfig {
    fn default() -> Self {
        Self { quality: 75 }
    }
}

#[cfg(feature = "image")]
impl<'a> crate::traits::ImageQualityConfig<'a> for JpegConfig {
    type Encoder = image::codecs::jpeg::JpegEncoder<&'a mut Vec<u8>>;
    fn get_encoder(&self, w: &'a mut Vec<u8>) -> Self::Encoder {
        image::codecs::jpeg::JpegEncoder::new_with_quality(w, self.quality)
    }
}

define_file!(
    Jpeg,
    "jpeg",
    [
        "image/jpeg",
        "image/jpg",
        "application/jpg",
        "application/x-jpg"
    ],
    ["jpeg", "jpg", "jpe", "jif", "jfif", "gfi"]
);
define_image_file!(Jpeg, image::ImageFormat::Jpeg);
define_custom_quality_image!(Jpeg, JpegConfig);

#[cfg(test)]
mod jpeg_tests {
    use std::env::temp_dir;

    use crate::{Temporary, traits::FsElement};

    use super::*;

    #[test]
    fn alt_extensions() {
        let dir = temp_dir();
        for ext in ["jpg", "jpe", "jif"] {
            let p = dir.join(format!("jpeg_alt.{}", ext));
            let f = Temporary::new(Jpeg::new(&p));
            f.create().unwrap();
            assert!(p.exists(), "failed for .{}", ext);
        }
    }
}
