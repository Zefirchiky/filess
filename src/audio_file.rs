use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{CodecParameters, Decoder, DecoderOptions, FinalizeResult};
use symphonia::core::formats::{FormatOptions, FormatReader, Packet, SeekMode, SeekTo};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::Metadata;
use symphonia::core::units::Time;

use crate::FileTrait;

#[derive(Debug, thiserror::Error)]
pub enum AudioError {
    #[error("Io error")]
    Io(#[from] std::io::Error),
    #[error("Symphonia error")]
    Symphonia(#[from] symphonia::core::errors::Error),
    #[error("No track was found in the file")]
    NoTrack,
}

pub struct DecodedStream<A: AudioFile, D: Decoder> {
    pub reader: A::Reader,
    pub decoder: D,
    pub track_id: u32,
    // Keep a reusable sample buffer to avoid re-allocating every frame
    sample_buf: Option<SampleBuffer<f32>>,
}

impl<A: AudioFile, D: Decoder> DecodedStream<A, D> {
    pub fn new(reader: A::Reader, decoder: D, track_id: u32) -> Self {
        Self {
            reader,
            decoder,
            track_id,
            sample_buf: None,
        }
    }

    /// Returns the next frame converted to f32 samples.
    pub fn next_frame(&mut self) -> Option<&[f32]> {
        let packet = self.reader.next_packet().ok()?;
        let decoded = self.decoder.decode(&packet).ok()?;

        // If this is the first frame, or format changed, initialize the SampleBuffer
        if let None = self.sample_buf {
            self.sample_buf = Some(SampleBuffer::new(decoded.capacity() as u64, *decoded.spec()));
        }
        
        let buf = self.sample_buf.as_mut()?;
        buf.copy_interleaved_ref(decoded);  // Normalize
        Some(buf.samples())
    }

    /// Jump to a specific second in the audio
    pub fn seek(&mut self, seconds: f64) -> Result<(), String> {
        self.reader.seek(
            SeekMode::Accurate,
            SeekTo::Time {
                time: Time::from(seconds),
                track_id: Some(self.track_id),
            },
        ).map_err(|e| e.to_string())?;
        
        self.decoder.reset();
        Ok(())
    }

    /// Access metadata (Title, Artist, etc.)
    pub fn metadata(&mut self) -> Metadata<'_> {
        self.reader.metadata()
    }
}

impl<A: AudioFile, D: Decoder> Iterator for DecodedStream<A, D> {
    type Item = Vec<f32>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_frame().map(|s| s.to_vec())
    }
}

pub struct DecodedStreamParams<R: FormatReader> {
    reader: R,
    params: CodecParameters,
    track_id: u32,
}

pub trait AudioFile: FileTrait {
    type Reader: FormatReader;
    
    fn load_audio_decoded_stream_params(&self) -> Result<DecodedStreamParams<Self::Reader>, AudioError> {
        let mss = MediaSourceStream::new(Box::new(self.as_file()?), Default::default());
        let reader = Self::Reader::try_new(mss, &FormatOptions::default())?;

        // Automatically find the first valid audio track
        let (track_id, params) = reader.tracks()
            .iter()
            .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
            .map(|t| (t.id, t.codec_params.clone()))
            .ok_or(AudioError::NoTrack)?;
        
        Ok(DecodedStreamParams { reader, params, track_id })
    }
}

pub trait AudioContainerFile: AudioFile {
    fn load_audio(&self) -> Result<DecodedStream<Self, DynamicDecoder>, AudioError> {
        let params = self.load_audio_decoded_stream_params()?;
        let decoder = symphonia::default::get_codecs()
                    .make(&params.params, &Default::default())?;
        Ok(DecodedStream::new(params.reader, DynamicDecoder(decoder), params.track_id))
    }
}

pub trait AudioCodecsFile: AudioFile {
    type Decoder: Decoder;
    fn codec_type() -> symphonia::core::codecs::CodecType;
    
    fn load_audio(&self) -> Result<DecodedStream<Self, Self::Decoder>, AudioError> {
        let params = self.load_audio_decoded_stream_params()?;
        let decoder = Self::Decoder::try_new(&params.params, &Default::default())?;
        Ok(DecodedStream::new(params.reader, decoder, params.track_id))
    }
}

pub struct DynamicDecoder(pub Box<dyn Decoder>);

impl Decoder for DynamicDecoder {
    fn decode(&mut self, packet: &Packet) -> symphonia::core::errors::Result<symphonia::core::audio::AudioBufferRef<'_>> {
        self.0.decode(packet)
    }
    fn reset(&mut self) { self.0.reset(); }
    fn codec_params(&self) -> &symphonia::core::codecs::CodecParameters { self.0.codec_params() }
    fn last_decoded(&self) -> symphonia::core::audio::AudioBufferRef<'_> { self.0.last_decoded() }
    fn finalize(&mut self) -> FinalizeResult { self.0.finalize() }
    fn try_new(_: &symphonia::core::codecs::CodecParameters, _: &DecoderOptions) -> symphonia::core::errors::Result<Self> where Self: Sized {
        Err(symphonia::core::errors::Error::Unsupported("Use constructor"))
    }
    fn supported_codecs() -> &'static [symphonia::core::codecs::CodecDescriptor] where Self: Sized { &[] }
}
