use crate::{define_file, define_image_file};

define_file!(Exr, "exr", ["image/x-exr"], ["exr"]);
define_image_file!(Exr, image::ImageFormat::OpenExr);
