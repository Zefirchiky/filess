use crate::{define_audio_container_file, define_audio_file, define_file};

define_file!(Mp4, ["mp4", "m4a", "mov"]);
define_audio_file!(Mp4, IsoMp4Reader);
define_audio_container_file!(Mp4);