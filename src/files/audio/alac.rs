use crate::define_file;

define_file!(Alac, ["caf"]);

#[cfg(feature = "audio")]
impl crate::AudioFile for Alac {
    type Reader = symphonia::default::formats::CafReader;
}

#[cfg(feature = "audio")]
impl crate::AudioCodecsFile for Alac {
    type Decoder = symphonia::default::codecs::AlacDecoder;
    fn codec_type() -> symphonia::core::codecs::CodecType { symphonia::core::codecs::CODEC_TYPE_ALAC }
}