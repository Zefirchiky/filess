use crate::{define_file, define_image_file};

define_file!(Qoi, "qoif", ["image/qoi"], ["qoi"]);
define_image_file!(Qoi, image::ImageFormat::Qoi);
