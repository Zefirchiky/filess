use crate::define_file;

define_file!(Aiff, ["aiff", "aif"]);

#[cfg(feature = "audio")]
impl crate::AudioFile for Aiff {
    type Reader = symphonia::default::formats::AiffReader;
}

#[cfg(feature = "audio")]
impl crate::AudioCodecsFile for Aiff {
    type Decoder = symphonia::default::codecs::PcmDecoder;
    fn codec_type() -> symphonia::core::codecs::CodecType { symphonia::core::codecs::CODEC_TYPE_PCM_S16BE }
}