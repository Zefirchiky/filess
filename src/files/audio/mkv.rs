use crate::define_file;

define_file!(Mkv, ["mkv", "mka", "webm"]);

#[cfg(feature = "audio")]
impl crate::AudioFile for Mkv {
    type Reader = symphonia::default::formats::MkvReader;
}

// Dynamically picks the decoder (Vorbis, Opus, etc.)
#[cfg(feature = "audio")]
impl crate::AudioContainerFile for Mkv {}