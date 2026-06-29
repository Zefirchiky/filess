use crate::{define_audio_container_file, define_audio_file, define_file};

define_file!(
    Ogg,
    "ogg",
    [
        "audio/ogg",
        "video/ogg",
        "application/ogg",
        "application/x-ogg",
        "audio/vorbis",
        "audio/opus",
        "audio/speex",
        "audio/vorbis-config",
        "audio/x-ogg",
        "video/theora"
    ],
    ["ogg", "opus"]
);
define_audio_file!(Ogg, OggReader);
define_audio_container_file!(Ogg);
