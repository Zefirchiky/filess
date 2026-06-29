use crate::{define_file, define_image_file};

define_file!(Ico, "ico", ["image/vnd.microsoft.icon", "image/x-icon"], ["ico"]);
define_image_file!(Ico, image::ImageFormat::Ico);