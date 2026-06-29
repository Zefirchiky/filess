use crate::{define_audio_container_file, define_audio_file, define_file};

define_file!(
    Mpa,
    "mpa",
    [
        "audio/mpeg",
        "audio/mpa",
        "application/octet-stream",
        "video/mpeg"
    ],
    ["mpa"]
);
define_audio_file!(Mpa, MpaReader);
define_audio_container_file!(Mpa);
