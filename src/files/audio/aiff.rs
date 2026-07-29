use crate::{define_audio_codecs_file, define_audio_file, define_file};

define_file!(
    Aiff,
    "aiff",
    [
        "audio/aiff",
        "audio/x-aiff",
        "audio/rmf",
        "audio/vnd.qcelp",
        "audio/x-gsm",
        "audio/x-midi",
        "audio/x-pn-aiff",
        "audio/x-rmf"
    ],
    ["aiff", "aif", "aifc"]
);
define_audio_file!(Aiff, AiffReader);
define_audio_codecs_file!(Aiff, PcmDecoder, CODEC_TYPE_PCM_S16BE);

#[cfg(all(test, feature = "audio"))]
mod aiff_tests {
    use crate::traits::AudioCodecsFile;
    use symphonia::core::codecs::CODEC_TYPE_PCM_S16BE;

    #[test]
    fn codec_type() {
        assert_eq!(super::Aiff::codec_type(), CODEC_TYPE_PCM_S16BE);
    }
}
