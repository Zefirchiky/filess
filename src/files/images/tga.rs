use crate::{define_file, define_image_file};

define_file!(Tga, "tga", ["image/x-tga", "image/x-targa"], ["tga"]);
define_image_file!(Tga, image::ImageFormat::Tga);
