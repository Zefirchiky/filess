use crate::{define_audio_codecs_file, define_audio_file, define_file};

define_file!(Flac, ["flac"]);
define_audio_file!(Flac, FlacReader);
define_audio_codecs_file!(Flac, FlacDecoder, CODEC_TYPE_FLAC);