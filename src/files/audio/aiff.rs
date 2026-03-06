use crate::{define_audio_codecs_file, define_audio_file, define_file};

define_file!(Aiff, ["aiff", "aif"]);
define_audio_file!(Aiff, AiffReader);
define_audio_codecs_file!(Aiff, PcmDecoder, CODEC_TYPE_PCM_S16BE);
