use crate::{define_audio_container_file, define_audio_file, define_file};

define_file!(Ogg, ["ogg", "opus"]);
define_audio_file!(Ogg, OggReader);
define_audio_container_file!(Ogg);