use crate::{define_audio_codecs_file, define_audio_file, define_file};

define_file!(Flac, "flac", ["audio/flac"], ["flac", "fla"]);
define_audio_file!(Flac, FlacReader);
define_audio_codecs_file!(Flac, FlacDecoder, CODEC_TYPE_FLAC);

#[cfg(all(test, feature = "audio"))]
mod flac_tests {
    use crate::traits::AudioCodecsFile;
    use symphonia::core::codecs::CODEC_TYPE_FLAC;

    #[test]
    fn codec_type() {
        assert_eq!(super::Flac::codec_type(), CODEC_TYPE_FLAC);
    }
}
