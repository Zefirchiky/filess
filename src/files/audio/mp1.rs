use crate::{define_audio_codecs_file, define_audio_file, define_file};

define_file!(Mp1, ["mp1"]);
define_audio_file!(Mp1, MpaReader);
define_audio_codecs_file!(Mp1, MpaDecoder, CODEC_TYPE_MP1);