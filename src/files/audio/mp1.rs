use crate::define_file;

define_file!(Mp1, ["mp1"]);

#[cfg(feature = "audio")]
impl crate::AudioFile for Mp1 {
    type Reader = symphonia::default::formats::MpaReader;
}

#[cfg(feature = "audio")]
impl crate::AudioCodecsFile for Mp1 {
    type Decoder = symphonia::default::codecs::MpaDecoder;
    fn codec_type() -> symphonia::core::codecs::CodecType { symphonia::core::codecs::CODEC_TYPE_MP1 }
}