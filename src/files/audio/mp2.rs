use crate::{define_audio_codecs_file, define_audio_file, define_file};

define_file!(
    Mp2,
    "mp2",
    [
        "audio/mpeg",
        "audio/x-mpeg",
        "audio/mpa",
        "video/mp2p",
        "video/mpeg"
    ],
    ["mp2", "m2a", "mp2a"]
);
define_audio_file!(Mp2, MpaReader);
define_audio_codecs_file!(Mp2, MpaDecoder, CODEC_TYPE_MP2);

#[cfg(all(test, feature = "audio"))]
mod mp2_tests {
    use crate::traits::AudioCodecsFile;
    use symphonia::core::codecs::CODEC_TYPE_MP2;

    #[test]
    fn codec_type() {
        assert_eq!(super::Mp2::codec_type(), CODEC_TYPE_MP2);
    }
}
