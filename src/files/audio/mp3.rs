use crate::{define_audio_codecs_file, define_audio_file, define_file};

define_file!(
    Mp3,
    "mp3",
    [
        "audio/mpeg",
        "audio/x-mpeg",
        "audio/mp3",
        "application/x-dtbncx+xml",
        "application/x-dtbook+xml",
        "audio/mpeg4-generic",
        "audio/x-wav",
        "text/xml",
        "image/mov",
        "image/x-quicktime",
        "video/quicktime",
        "video/x-quicktime",
        "audio/mpeg3",
        "audio/mpg",
        "audio/x-mp3",
        "audio/x-mpeg3",
        "audio/x-mpegaudio",
        "audio/x-mpg",
        "video/mpeg"
    ],
    ["mp3", "mpga"]
);
define_audio_file!(Mp3, MpaReader);
define_audio_codecs_file!(Mp3, MpaDecoder, CODEC_TYPE_MP3);

#[cfg(all(test, feature = "audio"))]
mod mp3_tests {
    use crate::traits::AudioCodecsFile;
    use symphonia::core::codecs::CODEC_TYPE_MP3;

    #[test]
    fn codec_type() {
        assert_eq!(super::Mp3::codec_type(), CODEC_TYPE_MP3);
    }
}
