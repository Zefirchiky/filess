use crate::{define_audio_codecs_file, define_audio_file, define_file};

define_file!(Alac, ["caf"]);
define_audio_file!(Alac, CafReader);
define_audio_codecs_file!(Alac, AlacDecoder, CODEC_TYPE_ALAC);