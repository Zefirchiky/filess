use crate::{define_file, define_image_file};

define_file!(Tiff, ["Tiff"], );
define_image_file!(Tiff, image::ImageFormat::Tiff);
