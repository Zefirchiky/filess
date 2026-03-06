use crate::{define_audio_codecs_file, define_audio_file, define_file};

define_file!(Mp3, ["mp3"]);
define_audio_file!(Mp3, MpaReader);
define_audio_codecs_file!(Mp3, MpaDecoder, CODEC_TYPE_MP3);