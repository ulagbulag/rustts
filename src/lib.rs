#[macro_use]
extern crate anyhow;

pub mod tts;
pub mod utils;

use anyhow::Result;
use tch::{IValue, Kind, Tensor};
use tts::SynthesisOptions;

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

    pub fn compute_spec(&self, filename: &str) -> Result<Tensor> {
        let y = crate::utils::audio::open_wav(filename)?;
        let spec = self.ap.spectrogram(y);
        let spec = spec.unsqueeze(0).to_kind(Kind::Float);
        Ok(spec)
    }

    pub fn embed(&self, files: &[impl AsRef<str>]) -> Result<Tensor> {
        self.speaker_manager.embed(files)
    }

    pub fn synthesis(
        &self,
        text: &str,
        speaker_emb: &Tensor,
        options: &SynthesisOptions,
    ) -> Result<Tensor> {
        // preprocess the given text
        let text_inputs = self.config.text_to_sequence(text).unsqueeze(0);

        if let IValue::Tensor(model_outputs) =
            self.model.inference(&text_inputs, speaker_emb, options)?
        {
            Ok(model_outputs.squeeze())
        } else {
            unreachable!()
        }
    }

    pub fn voice_conversion(
        &self,
        x: &Tensor,
        speaker_cond_src: &Tensor,
        speaker_cond_tgt: &Tensor,
    ) -> Result<Tensor> {
        let ref_wav = self
            .model
            .voice_conversion(x, speaker_cond_src, speaker_cond_tgt)?;

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
