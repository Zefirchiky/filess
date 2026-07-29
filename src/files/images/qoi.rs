use crate::{define_file, define_image_file};

define_file!(Qoi, "qoi", ["image/qoi"], ["qoi", "qoif"]);
define_image_file!(Qoi, image::ImageFormat::Qoi);
