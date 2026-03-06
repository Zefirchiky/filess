use crate::{define_audio_container_file, define_audio_file, define_file};

define_file!(Mkv, ["mkv", "mka", "webm"]);
define_audio_file!(Mkv, MkvReader);
define_audio_container_file!(Mkv);