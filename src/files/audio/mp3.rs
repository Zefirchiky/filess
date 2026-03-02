use crate::define_file;

define_file!(Mp3, ["mp3"]);

#[cfg(feature = "audio")]
impl crate::AudioFile for Mp3 {
    type Reader = symphonia::default::formats::MpaReader;
}

#[cfg(feature = "audio")]
impl crate::AudioCodecsFile for Mp3 {
    type Decoder = symphonia::default::codecs::MpaDecoder;
    fn codec_type() -> symphonia::core::codecs::CodecType { symphonia::core::codecs::CODEC_TYPE_MP3 }
}