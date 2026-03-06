use crate::{define_audio_codecs_file, define_audio_file, define_file};

define_file!(Mp2, ["mp2"]);
define_audio_file!(Mp2, MpaReader);
define_audio_codecs_file!(Mp2, MpaDecoder, CODEC_TYPE_MP2);