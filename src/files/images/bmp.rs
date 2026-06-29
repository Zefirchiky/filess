use crate::{define_file, define_image_file};

define_file!(
    Bmp,
    "bmp",
    [
        "image/bmp",
        "image/x-ms-bmp",
        "image/x-bmp",
        "application/octet-stream",
        "image/x-award-bioslogo2",
        "image/x-award-bioslogo"
    ],
    ["bmp", "dib"]
);
define_image_file!(Bmp, image::ImageFormat::Bmp);
