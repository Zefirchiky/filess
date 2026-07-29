use crate::{define_audio_codecs_file, define_audio_file, define_file};

define_file!(
    Mp1,
    "mp1",
    ["audio/mpeg", "audio/mpa", "video/mp1s", "video/mpeg"],
    ["mp1"]
);
define_audio_file!(Mp1, MpaReader);
define_audio_codecs_file!(Mp1, MpaDecoder, CODEC_TYPE_MP1);

#[cfg(all(test, feature = "audio"))]
mod mp1_tests {
    use crate::traits::AudioCodecsFile;
    use symphonia::core::codecs::CODEC_TYPE_MP1;

    #[test]
    fn codec_type() {
        assert_eq!(super::Mp1::codec_type(), CODEC_TYPE_MP1);
    }
}
