use crate::{define_file, define_image_file};

define_file!(
    Hdr,
    "hdr",
    ["image/vnd.radiance", "application/octet-stream"],
    ["hdr"]
);
define_image_file!(Hdr, image::ImageFormat::Hdr);
