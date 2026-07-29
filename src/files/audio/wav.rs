use crate::{define_audio_codecs_file, define_audio_file, define_file};

define_file!(
    Wav,
    "wav",
    [
        "audio/wav",
        "audio/x-wav",
        "audio/vnd.wave",
        "audio/wave",
        "audio/x-pn-wav",
        "application/x-dtbncx+xml",
        "application/x-dtbook+xml",
        "audio/mpeg",
        "audio/mpeg4-generic",
        "text/xml"
    ],
    ["wav"]
);
define_audio_file!(Wav, WavReader);
define_audio_codecs_file!(Wav, PcmDecoder, CODEC_TYPE_PCM_S16LE);

#[cfg(all(test, feature = "audio"))]
mod wav_tests {
    use crate::traits::AudioCodecsFile;
    use symphonia::core::codecs::CODEC_TYPE_PCM_S16LE;

    #[test]
    fn codec_type() {
        assert_eq!(super::Wav::codec_type(), CODEC_TYPE_PCM_S16LE);
    }
}
