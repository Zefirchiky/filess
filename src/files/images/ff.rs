use crate::{define_file, define_image_file};

define_file!(
    Ff,
    "ff",
    ["application/octet-stream", "image/x-farbfeld"],
    ["ff"]
);
define_image_file!(Ff, image::ImageFormat::Farbfeld);
