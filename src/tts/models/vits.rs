use anyhow::Result;
use tch::{CModule, IValue, Tensor};

use crate::utils::config::Config;

pub struct Vits {
    model: CModule,
}

impl Vits {
    pub fn try_new(model_path: &str) -> Result<Self> {
        Ok(Self {
            model: CModule::load(model_path)?,
        })
    }

    pub fn make_symbols<'a>(&self, config: &'a Config) -> Vec<&'a str> {
        let pad = &config.characters.pad;
        let punctuations = &config.characters.punctuations;
        let letters = &config.characters.characters;
        if config.use_phonemes {
            let letters_ipa = &config.characters.phonemes;
            vec![pad, punctuations, letters, letters_ipa]
        } else {
            vec![pad, punctuations, letters]
        }
    }

    pub fn voice_conversion(
        &self,
        y: &Tensor,
        y_lengths: &Tensor,
        speaker_cond_src: &Tensor,
        speaker_cond_tgt: &Tensor,
    ) -> Result<IValue> {
        {
            let y: IValue = y.detach().into();
            let y_lengths: IValue = y_lengths.detach().into();
            let speaker_cond_src: IValue = speaker_cond_src.detach().into();
            let speaker_cond_tgt: IValue = speaker_cond_tgt.detach().into();
            self.model
                .forward_is(&[y, y_lengths, speaker_cond_src, speaker_cond_tgt])
                .map_err(Into::into)
        }

        // let num_speakers = 1i64;
        // let use_speaker_embedding = true;
        // let use_d_vector = true;

        // if num_speakers <= 0 {
        //     bail!("num_speakers have to be larger than 0.");
        // }

        // // speaker embedding
        // let (g_src, g_tgt) = if use_speaker_embedding {
        //     if use_d_vector {
        //         let g_src = speaker_cond_src.norm().unsqueeze(-1);
        //         let g_tgt = speaker_cond_tgt.norm().unsqueeze(-1);
        //         (g_src, g_tgt)
        //     } else {
        //         let g_src: Tensor = {
        //             let speaker_cond_src: IValue = speaker_cond_src.detach().into();
        //             self.model
        //                 .method_is("emb_g", &[speaker_cond_src])?
        //                 .try_into()?
        //         };
        //         let g_src = g_src.unsqueeze(-1);
        //         let g_tgt: Tensor = {
        //             let speaker_cond_tgt: IValue = speaker_cond_tgt.detach().into();
        //             self.model
        //                 .method_is("emb_g", &[speaker_cond_tgt])?
        //                 .try_into()?
        //         };
        //         let g_tgt = g_tgt.unsqueeze(-1);
        //         (g_src, g_tgt)
        //     }
        // } else {
        //     bail!(" [!] Voice conversion is only supported on multi-speaker models.")
        // };

        // let y: IValue = y.detach().into();
        // let y_lengths: IValue = y_lengths.detach().into();
        // let g_src: IValue = g_src.detach().into();
        // let g_tgt: IValue = g_tgt.detach().into();

        // let (z, y_mask) = if let IValue::Tuple(mut out) = self
        //     .model
        //     .method_is("posterior_encoder", &[&y, &y_lengths, &g_src])?
        // {
        //     let y_mask: Tensor = out.pop().unwrap().try_into()?;
        //     let _ = out.pop().unwrap();
        //     let _ = out.pop().unwrap();
        //     let z = out.pop().unwrap();
        //     (z, y_mask)
        // } else {
        //     unreachable!()
        // };
        // let z_p = {
        //     let y_mask = y_mask.detach().into();
        //     let reverse = IValue::Bool(false);
        //     self.model
        //         .method_is("flow", &[&z, &y_mask, &g_src, &reverse])?
        // };
        // let z_hat: Tensor = {
        //     let y_mask = y_mask.detach().into();
        //     let reverse = IValue::Bool(true);
        //     self.model
        //         .method_is("flow", &[&z_p, &y_mask, &g_tgt, &reverse])?
        //         .try_into()?
        // };
        // let o_hat = {
        //     let z_hat = z_hat.detach();
        //     let y_mask = y_mask.detach();
        //     self.model
        //         .method_is("waveform_decoder", &[(z_hat * y_mask).into(), g_tgt])?
        // };
        // Ok(IValue::Tuple(vec![
        //     o_hat,
        //     y_mask.into(),
        //     IValue::Tuple(vec![z, z_p, z_hat.into()]),
        // ]))
    }
}
