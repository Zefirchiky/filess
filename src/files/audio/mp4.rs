use crate::define_file;

define_file!(Mp4, ["mp4", "m4a", "mov"]);

#[cfg(feature = "audio")]
impl crate::AudioFile for Mp4 {
    type Reader = symphonia::default::formats::IsoMp4Reader;
}

#[cfg(feature = "audio")]
impl crate::AudioContainerFile for Mp4 {}