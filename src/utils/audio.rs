use anyhow::Result;
use tch::{Device, Kind, Tensor};

pub struct AudioProcessor {
    pub(crate) config: AudioConfig,
}

impl AudioProcessor {
    pub fn try_new(config: AudioConfig) -> Result<Self> {
        Ok(Self { config })
    }

    pub fn spectrogram(&self, y: Tensor) -> Tensor {
        let d = if self.config.preemphasis != 0.0 {
            // TODO: to be implemented
            todo!()
        } else {
            self.stft(y)
        };
        let s = if self.config.do_amp_to_db_linear {
            // TODO: to be implemented
            todo!()
        } else {
            d.abs()
        };
        self.normalize(s)
    }

    fn stft(&self, waveform: Tensor) -> Tensor {
        const HOP_MARGIN: i64 = 3;

        let win_length = self.config.win_length.unwrap_or(self.config.fft_size);

        let window = tch::Tensor::hann_window(win_length, (Kind::Double, Device::Cpu));

        waveform.stft(
            self.config.fft_size,
            self.config
                .hop_length
                .map(|hop_length| hop_length - HOP_MARGIN),
            self.config.win_length,
            Some(window),
            false,
            true,
            true,
        )
    }

    fn normalize(&self, s: Tensor) -> Tensor {
        if self.config.signal_norm.unwrap_or(false) {
            // TODO: to be implemented
            todo!()
        } else {
            s
        }
    }
}

#[derive(Clone, Debug)]
pub struct AudioConfig {
    pub sample_rate: Option<i64>,
    pub resample: bool,
    pub num_mels: Option<i64>,
    pub min_level_db: Option<i64>,
    pub frame_shift_ms: Option<i64>,
    pub frame_length_ms: Option<i64>,
    pub hop_length: Option<i64>,
    pub win_length: Option<i64>,
    pub ref_level_db: Option<i64>,
    pub fft_size: i64,
    pub power: Option<f64>,
    pub preemphasis: f64,
    pub signal_norm: Option<bool>,
    pub symmetric_norm: Option<bool>,
    pub max_norm: Option<f64>,
    pub mel_fmin: Option<f64>,
    pub mel_fmax: Option<f64>,
    pub spec_gain: i64,
    pub stft_pad_mode: String,
    pub clip_norm: bool,
    pub griffin_lim_iters: Option<i64>,
    pub do_trim_silence: bool,
    pub trim_db: i64,
    pub do_sound_norm: bool,
    pub do_amp_to_db_linear: bool,
    pub do_amp_to_db_mel: bool,
    pub stats_path: Option<String>,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: None,
            resample: false,
            num_mels: None,
            min_level_db: None,
            frame_shift_ms: None,
            frame_length_ms: None,
            hop_length: None,
            win_length: None,
            ref_level_db: None,
            fft_size: 1024,
            power: None,
            preemphasis: 0.0,
            signal_norm: None,
            symmetric_norm: None,
            max_norm: None,
            mel_fmin: None,
            mel_fmax: None,
            spec_gain: 20,
            stft_pad_mode: "reflect".to_string(),
            clip_norm: true,
            griffin_lim_iters: None,
            do_trim_silence: false,
            trim_db: 60,
            do_sound_norm: false,
            do_amp_to_db_linear: true,
            do_amp_to_db_mel: true,
            stats_path: None,
        }
    }
}

impl AudioConfig {
    pub fn with_vits() -> Self {
        Self {
            sample_rate: Some(16000),
            resample: false,
            num_mels: Some(80),
            min_level_db: Some(-100),
            frame_shift_ms: None,
            frame_length_ms: None,
            hop_length: Some(256),
            win_length: Some(1024),
            ref_level_db: Some(20),
            fft_size: 1024,
            power: Some(1.5),
            preemphasis: 0.0,
            signal_norm: Some(false),
            symmetric_norm: Some(true),
            max_norm: Some(4.0),
            mel_fmin: Some(0.0),
            mel_fmax: None,
            spec_gain: 1,
            stft_pad_mode: "reflect".to_string(),
            clip_norm: true,
            griffin_lim_iters: Some(60),
            do_trim_silence: true,
            trim_db: 45,
            do_sound_norm: false,
            do_amp_to_db_linear: false,
            do_amp_to_db_mel: true,
            stats_path: None,
        }
    }

    pub fn with_speaker_encoder() -> Self {
        Self {
            sample_rate: Some(16000),
            resample: false,
            num_mels: Some(64),
            min_level_db: Some(-100),
            frame_shift_ms: None,
            frame_length_ms: None,
            hop_length: Some(160),
            win_length: Some(400),
            ref_level_db: Some(20),
            fft_size: 512,
            power: Some(1.5),
            preemphasis: 0.97,
            signal_norm: Some(false),
            symmetric_norm: Some(false),
            max_norm: Some(4.0),
            mel_fmin: Some(0.0),
            mel_fmax: Some(8000.0),
            spec_gain: 20,
            stft_pad_mode: "reflect".to_string(),
            clip_norm: false,
            griffin_lim_iters: Some(60),
            do_trim_silence: false,
            trim_db: 60,
            do_sound_norm: false,
            stats_path: None,
            ..Default::default()
        }
    }
}

pub fn open_wav(filename: &str) -> Result<Tensor> {
    hound::WavReader::open(filename)?
        .samples()
        .map(|sample: Result<i16, _>| {
            sample
                .map(|sample| sample as f32 / 32767. * 1.414)
                .map_err(Into::into)
        })
        .collect::<Result<Vec<_>>>()
        .map(|ref waveform| Tensor::of_slice(waveform))
}

pub fn save_wav(ref_wav_voc: Tensor, filename: &str) -> Result<()> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(filename, spec)?;
    for t in ref_wav_voc.iter::<f64>()? {
        writer.write_sample((t / 1.414 * 32767.) as i16)?;
    }
    Ok(())
}
