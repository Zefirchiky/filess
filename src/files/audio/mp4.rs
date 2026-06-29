use crate::{define_audio_container_file, define_audio_file, define_file};

define_file!(
    Mp4,
    "mp4",
    [
        "video/mp4",
        "audio/mp4",
        "application/mp4",
        "application/mpeg4-iod",
        "application/mpeg4-iod-xmt",
        "video/mp4v-es",
        "video/mpeg4-generic",
        "application/x-dtbncx+xml",
        "application/x-dtbook+xml",
        "audio/mpeg",
        "audio/mpeg4-generic",
        "audio/x-wav",
        "text/xml",
        "application/octet-stream",
        "audio/aac",
        "audio/aacp",
        "audio/mp4a",
        "audio/mp4a-latm",
        "audio/mpga",
        "audio/x-aac",
        "audio/x-m4a",
        "audio/x-m4b",
        "audio/x-m4p",
        "audio/x-mp4a-latm"
    ],
    ["mp4", "m4a", "m4p", "m4b", "m4v", "mov"]
);
define_audio_file!(Mp4, IsoMp4Reader);
define_audio_container_file!(Mp4);
