use crate::{define_file, define_image_file};

define_file!(WebP, ["webp"], );
define_image_file!(WebP, image::ImageFormat::WebP);
