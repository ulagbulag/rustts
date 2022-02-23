use anyhow::Result;
use tch::{CModule, IValue, IndexOp, Tensor};

use crate::audio::{self, AudioConfig};

pub struct SpeakerManager {
    audio_config: AudioConfig,
    speaker_encoder: CModule,
}

impl SpeakerManager {
    pub fn try_new(speaker_encoder_path: &str) -> Result<Self> {
        Ok(Self {
            audio_config: AudioConfig::with_speaker_encoder(),
            speaker_encoder: CModule::load(speaker_encoder_path)?,
        })
    }

    pub fn embed(&self, files: &[&str]) -> Result<Tensor> {
        if files.is_empty() {
            bail!("No files were given.");
        }

        let mut d_vectors: Option<Tensor> = None;
        for filename in files {
            let waveform = audio::open_wav(filename)?;
            let waveform = waveform.unsqueeze(0);

            let d_vector = self.compute_embedding(&waveform)?;

            d_vectors = Some(match d_vectors {
                Some(d_vectors) => d_vectors + d_vector,
                None => d_vector,
            });
        }

        Ok(d_vectors.unwrap() / files.len() as i64)
    }

    fn compute_embedding(&self, x: &Tensor) -> Result<Tensor> {
        let use_torch_spec = true;
        let num_frames = 250;
        let num_eval = 10;
        let return_mean = true;

        let num_frames = match self.audio_config.hop_length {
            Some(hop_length) if use_torch_spec => num_frames * hop_length,
            _ => num_frames,
        };

        let max_len = x.size()[1];

        let num_frames = num_frames.min(max_len);

        let offsets = ndarray::Array1::linspace(0f64, (max_len - num_frames) as f64, num_eval);

        let frames_batch: Vec<_> = offsets
            .into_iter()
            .map(|offset| {
                let offset = offset as i64;
                let end_offset = offset + num_frames;
                x.i((.., offset..end_offset))
            })
            .collect();

        let frames_batch = tch::Tensor::cat(&frames_batch, 0);
        let frames_batch: IValue = frames_batch.into();
        let embedding = self.speaker_encoder.forward_is(&[frames_batch])?;
        let embedding: Tensor = embedding.try_into()?;

        if return_mean {
            Ok(embedding.mean_dim(&[0], true, embedding.kind()))
        } else {
            Ok(embedding)
        }
    }
}
