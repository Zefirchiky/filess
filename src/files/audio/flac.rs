use crate::define_file;

define_file!(Flac, ["flac"]);

#[cfg(feature = "audio")]
impl crate::AudioFile for Flac {
    type Reader = symphonia::default::formats::FlacReader;
}

#[cfg(feature = "audio")]
impl crate::AudioCodecsFile for Flac {
    type Decoder = symphonia::default::codecs::FlacDecoder;
    fn codec_type() -> symphonia::core::codecs::CodecType { symphonia::core::codecs::CODEC_TYPE_FLAC }
}