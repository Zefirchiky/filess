use crate::{define_audio_codecs_file, define_audio_file, define_file};

define_file!(Wav, ["wav"]);
define_audio_file!(Wav, WavReader);
define_audio_codecs_file!(Wav, PcmDecoder, CODEC_TYPE_PCM_S16LE);