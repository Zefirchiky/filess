use crate::{define_audio_codecs_file, define_audio_file, define_file};

define_file!(Alac, "alac", [], ["caf"]);
define_audio_file!(Alac, CafReader);
define_audio_codecs_file!(Alac, AlacDecoder, CODEC_TYPE_ALAC);

#[cfg(all(test, feature = "audio"))]
mod alac_tests {
    use crate::traits::AudioCodecsFile;
    use symphonia::core::codecs::CODEC_TYPE_ALAC;

    #[test]
    fn codec_type() {
        assert_eq!(super::Alac::codec_type(), CODEC_TYPE_ALAC);
    }
}
