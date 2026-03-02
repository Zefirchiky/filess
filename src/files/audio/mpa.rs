use crate::define_file;

define_file!(Mpa, ["mpa"]);

#[cfg(feature = "audio")]
impl crate::AudioFile for Mpa {
    type Reader = symphonia::default::formats::MpaReader;
}

#[cfg(feature = "audio")]
impl crate::AudioContainerFile for Mpa {}