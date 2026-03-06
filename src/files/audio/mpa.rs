use crate::{define_audio_container_file, define_audio_file, define_file};

define_file!(Mpa, ["mpa"]);
define_audio_file!(Mpa, MpaReader);
define_audio_container_file!(Mpa);
