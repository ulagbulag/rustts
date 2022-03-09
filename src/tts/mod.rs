pub mod models;

#[derive(Clone, Debug)]
pub struct SynthesisOptions {
    pub language_id: i64,
    pub length_scale: f64,
    pub inference_noise_scale: f64,
    pub max_inference_len: Option<i64>,
}

impl Default for SynthesisOptions {
    fn default() -> Self {
        Self {
            language_id: 0,
            length_scale: 1.0,
            inference_noise_scale: 0.0,
            max_inference_len: None,
        }
    }
}
