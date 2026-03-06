use crate::define_file;

define_file!(Image, [""]);

#[cfg(feature = "image")]
impl crate::traits::ImageFile for Image {
    fn image_format() -> image::ImageFormat {
        image::ImageFormat::Avif
    }
}
