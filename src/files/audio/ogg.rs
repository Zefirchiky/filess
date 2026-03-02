use crate::define_file;

define_file!(Ogg, ["ogg", "opus"]);

#[cfg(feature = "audio")]
impl crate::AudioFile for Ogg {
    type Reader = symphonia::default::formats::OggReader;
}

#[cfg(feature = "audio")]
impl crate::AudioContainerFile for Ogg {}