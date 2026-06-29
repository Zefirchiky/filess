use crate::{define_file, define_image_file};

define_file!(
    Tiff,
    "tiff",
    [
        "image/tiff",
        "image/dng",
        "image/tiff-fx",
        "application/tif",
        "application/tiff",
        "application/x-tif",
        "application/x-tiff",
        "image/tif",
        "image/x-tif",
        "image/x-tiff",
        "application/octet-stream"
    ],
    ["tiff", "tif"]
);
define_image_file!(Tiff, image::ImageFormat::Tiff);
