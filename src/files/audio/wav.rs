use crate::define_file;

define_file!(Wav, ["wav"]);

#[cfg(feature = "audio")]
impl crate::AudioFile for Wav {
    type Reader = symphonia::default::formats::WavReader;
}

// We use AudioCodecsFile because WAV -> PCM is a 1:1 mapping we can count on
#[cfg(feature = "audio")]
impl crate::AudioCodecsFile for Wav {
    type Decoder = symphonia::default::codecs::PcmDecoder;
    fn codec_type() -> symphonia::core::codecs::CodecType {
        // Standard PCM
        symphonia::core::codecs::CODEC_TYPE_PCM_S16LE
    }
}