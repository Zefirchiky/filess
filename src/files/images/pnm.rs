//pbm, pgm, ppm and pam
use crate::{define_file, define_image_file};

define_file!(
    Pnm,
    "pnm",
    [
        "image/x-portable-anymap",
        "image/x-portable-bitmap",
        "image/x-portable-graymap",
        "image/x-portable-pixmap",
        "image/x-portable-floatmap",
        "text/plain"
    ],
    ["pnm", "pbm", "ppm", "pam"]
);
define_image_file!(Pnm, image::ImageFormat::Pnm);
