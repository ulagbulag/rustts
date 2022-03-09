use std::path::PathBuf;

use anyhow::Result;
use tch::{CModule, IValue, Kind, Scalar, Tensor};

use crate::utils::functional::normalize;

pub struct Vits {
    // TODO: Use TorchScript instead (so that we can remove the dedicated reversed model)
    duration_predictor_reversed: CModule,
    emb_l: CModule,
    flow: CModule,
    // TODO: Use TorchScript instead (so that we can remove the dedicated reversed model)
    flow_reversed: CModule,
    posterior_encoder: CModule,
    text_encoder: CModule,
    waveform_decoder: CModule,
}

impl Vits {
    pub fn try_new(model_path: impl Into<PathBuf>) -> Result<Self> {
        let model_path = model_path.into();

        Ok(Self {
            duration_predictor_reversed: CModule::load(
                model_path.join("duration_predictor_reversed.pt"),
            )?,
            emb_l: CModule::load(model_path.join("emb_l.pt"))?,
            flow: CModule::load(model_path.join("flow.pt"))?,
            flow_reversed: CModule::load(model_path.join("flow_reversed.pt"))?,
            posterior_encoder: CModule::load(model_path.join("posterior_encoder.pt"))?,
            text_encoder: CModule::load(model_path.join("text_encoder.pt"))?,
            waveform_decoder: CModule::load(model_path.join("waveform_decoder.pt"))?,
        })
    }

    pub fn inference(
        &self,
        x: &Tensor,
        speaker_emb: &Tensor,
        language_id: &Tensor,
    ) -> Result<IValue> {
        fn sequence_mask(sequence_length: &Tensor, max_len: impl Into<Scalar>) -> Tensor {
            let seq_range =
                Tensor::arange(max_len, (sequence_length.kind(), sequence_length.device()));
            // B x T_max
            seq_range
                .unsqueeze(0)
                .lt_tensor(&sequence_length.unsqueeze(1))
        }

        fn generate_path(duration: &Tensor, mask: &Tensor) -> Tensor {
            fn convert_pad_shape(pad_shape: &[&[i64]]) -> Vec<i64> {
                pad_shape.iter().rev().flat_map(|e| e.to_vec()).collect()
            }

            let kind = mask.kind();
            let (b, t_x, t_y) = {
                let mut shape = mask.size();
                let t_y = shape.pop().unwrap();
                let t_x = shape.pop().unwrap();
                let b = shape.pop().unwrap();
                (b, t_x, t_y)
            };
            let cum_duration = duration.cumsum(1, kind);

            let cum_duration_flat = cum_duration.view(b * t_x);
            let path = sequence_mask(&cum_duration_flat, t_y).to_dtype(kind, false, false);
            let path = path.view([b, t_x, t_y]);
            let path = (&path)
                - path
                    .constant_pad_nd(&convert_pad_shape(&[&[0, 0], &[1, 0], &[0, 0]]))
                    .slice(1, 0, -1, 1);
            path * mask
        }

        let length_scale: f64 = 1.0;
        let inference_noise_scale: f64 = 0.0;
        let max_inference_len = None;

        let x_lengths = tch::Tensor::of_slice(&[*x.size().last().unwrap()]);

        let x = x.detach().into();
        let x_lengths = x_lengths.into();

        let g = speaker_emb.unsqueeze(-1).into();
        let lid = language_id.detach().into();

        // language embedding
        let lang_emb = {
            let lang_emb: Tensor = self.emb_l.forward_is(&[&lid])?.try_into()?;
            lang_emb.unsqueeze(-1)
        };

        let (x, m_p, logs_p, x_mask) = if let IValue::Tuple(mut out) = {
            let lang_emb = lang_emb.detach().into();
            self.text_encoder.forward_is(&[&x, &x_lengths, &lang_emb])?
        } {
            let x_mask: Tensor = out.pop().unwrap().try_into()?;
            let logs_p: Tensor = out.pop().unwrap().try_into()?;
            let m_p: Tensor = out.pop().unwrap().try_into()?;
            let x: Tensor = out.pop().unwrap().try_into()?;
            (x, m_p, logs_p, x_mask)
        } else {
            unreachable!()
        };

        // use sdp
        let logw: Tensor = {
            let x = x.detach().into();
            let x_mask = x_mask.detach().into();
            let dr = Tensor::zeros_like(language_id).into();
            let lang_emb = lang_emb.into();
            let reverse = Tensor::of_slice(&[true]).into();
            let noise_scale = Tensor::of_slice(&[inference_noise_scale as f32]).into();

            self.duration_predictor_reversed
                .forward_is(&[
                    &x,
                    &x_lengths,
                    &x_mask,
                    &dr,
                    &g,
                    &lang_emb,
                    &reverse,
                    &noise_scale,
                ])?
                .try_into()?
        };

        let w = logw.exp() * (&x_mask) * length_scale;
        let w_ceil = w.ceil();
        let y_lengths = w_ceil
            .sum_dim_intlist(&[1, 2], false, w_ceil.kind())
            .clamp_min(1)
            .to_dtype(Kind::Int64, false, false);
        let y_mask = {
            let max_len: i64 = y_lengths.max().into();
            sequence_mask(&y_lengths, max_len).to(x_mask.device())
        };
        let attn_mask = x_mask.unsqueeze(2) * y_mask.unsqueeze(-1);
        let attn = generate_path(
            &w_ceil.squeeze_dim(1),
            &attn_mask.squeeze_dim(1).transpose(1, 2),
        );

        let m_p = attn
            .transpose(1, 2)
            .matmul(&m_p.transpose(1, 2))
            .transpose(1, 2);
        let logs_p = attn
            .transpose(1, 2)
            .matmul(&logs_p.transpose(1, 2))
            .transpose(1, 2);

        let z_p = (&m_p) + Tensor::randn_like(&m_p) * logs_p.exp() * inference_noise_scale;
        let z: Tensor = {
            let z_p = z_p.into();
            let y_mask = y_mask.detach().into();
            let reverse = Tensor::of_slice(&[true]).into();
            self.flow_reversed
                .forward_is(&[&z_p, &y_mask, &g, &reverse])?
                .try_into()?
        };
        {
            let input = (z * y_mask).slice(2, 0, max_inference_len, 1).into();
            self.waveform_decoder
                .forward_is(&[&input, &g])
                .map_err(Into::into)
        }
    }

    pub fn voice_conversion(
        &self,
        x: &Tensor,
        speaker_cond_src: &Tensor,
        speaker_cond_tgt: &Tensor,
    ) -> Result<IValue> {
        let x_lengths = tch::Tensor::of_slice(&[*x.size().last().unwrap()]);

        // speaker embedding
        let g_src = normalize(speaker_cond_src).unsqueeze(-1);
        let g_tgt = normalize(speaker_cond_tgt).unsqueeze(-1);

        let x: IValue = x.detach().into();
        let x_lengths: IValue = x_lengths.detach().into();
        let g_src: IValue = g_src.detach().into();
        let g_tgt: IValue = g_tgt.detach().into();

        let (z, y_mask) = if let IValue::Tuple(mut out) = self
            .posterior_encoder
            .forward_is(&[&x, &x_lengths, &g_src])?
        {
            let y_mask: Tensor = out.pop().unwrap().try_into()?;
            let _ = out.pop().unwrap();
            let _ = out.pop().unwrap();
            let z = out.pop().unwrap();
            (z, y_mask)
        } else {
            unreachable!()
        };
        let z_p = {
            let y_mask = y_mask.detach().into();
            let reverse = Tensor::of_slice(&[false]).into();
            self.flow.forward_is(&[&z, &y_mask, &g_src, &reverse])?
        };
        let z_hat: Tensor = {
            let y_mask = y_mask.detach().into();
            let reverse = Tensor::of_slice(&[true]).into();
            self.flow_reversed
                .forward_is(&[&z_p, &y_mask, &g_tgt, &reverse])?
                .try_into()?
        };
        let o_hat = {
            let z_hat = z_hat.detach();
            let y_mask = y_mask.detach();
            self.waveform_decoder
                .forward_is(&[(z_hat * y_mask).into(), g_tgt])?
        };
        Ok(IValue::Tuple(vec![
            o_hat,
            y_mask.into(),
            IValue::Tuple(vec![z, z_p, z_hat.into()]),
        ]))
    }
}
