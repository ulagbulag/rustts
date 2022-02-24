#[macro_use]
extern crate anyhow;

pub mod tts;
pub mod utils;

use anyhow::Result;
use tch::{IValue, Kind, Tensor};

pub struct TTS {
    ap: crate::utils::audio::AudioProcessor,
    config: crate::utils::config::Config,
    model: crate::tts::models::vits::Vits,
    speaker_manager: crate::utils::speakers::SpeakerManager,
}

impl TTS {
    pub fn try_default(model_path: &str, speaker_encoder_path: &str) -> Result<Self> {
        let config = crate::utils::config::Config {
            audio: crate::utils::audio::AudioConfig::with_vits(),
            ..Default::default()
        };

        Ok(Self {
            ap: crate::utils::audio::AudioProcessor::try_new(config.audio.clone())?,
            config,
            model: crate::tts::models::vits::Vits::try_new(model_path)?,
            speaker_manager: crate::utils::speakers::SpeakerManager::try_new(speaker_encoder_path)?,
        })
    }

    pub fn compute_spec(&self, filename: &str) -> Result<(Tensor, Tensor)> {
        let y = crate::utils::audio::open_wav(filename)?;
        let spec = self.ap.spectrogram(y);
        let spec = spec.unsqueeze(0).to_kind(Kind::Float);
        let y_lengths = tch::Tensor::of_slice(&[*spec.size().last().unwrap()]);
        Ok((spec, y_lengths))
    }

    pub fn embed(&self, files: &[&str]) -> Result<Tensor> {
        self.speaker_manager.embed(files)
    }

    pub fn synthesis(&self) -> Result<()> {
        // GST processing
        let custom_symbols = self.model.make_symbols(&self.config);

        // preprocess the given text
        todo!()
    }

    pub fn voice_conversion(
        &self,
        driving_spec: &Tensor,
        y_lengths: &Tensor,
        driving_emb: &Tensor,
        target_emb: &Tensor,
    ) -> Result<Tensor> {
        let ref_wav =
            self.model
                .voice_conversion(&driving_spec, &y_lengths, &driving_emb, &target_emb)?;

        let ref_wav_voc = if let IValue::Tuple(mut ref_wav) = ref_wav {
            if let IValue::Tuple(mut z_tuple) = ref_wav.pop().unwrap() {
                let _z: Tensor = z_tuple.pop().unwrap().try_into()?;
                let _z_p: Tensor = z_tuple.pop().unwrap().try_into()?;
                let _z_hat: Tensor = z_tuple.pop().unwrap().try_into()?;
            }
            let _y_mask: Tensor = ref_wav.pop().unwrap().try_into()?;
            let _o_hat: Tensor = ref_wav.pop().unwrap().try_into()?;
            _o_hat
        } else {
            unreachable!()
        };

        Ok(ref_wav_voc.squeeze().squeeze())
    }
}
