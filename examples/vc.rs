use anyhow::Result;

fn main() -> Result<()> {
    // Load a Model
    let model_path = "./assets/vits";
    let speaker_encoder_path = "./assets/speaker_encoder.pt";
    let tts = rustts::TTS::try_default(model_path, speaker_encoder_path)?;

    // Command-line Arguments
    let x = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "./assets/samples/speaker_man_korean/000000.wav".to_string());
    let speaker_cond_src = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "./assets/samples/speaker_man_korean/".to_string());
    let speaker_cond_tgt = std::env::args()
        .nth(3)
        .unwrap_or_else(|| "./assets/samples/speaker_woman_english/".to_string());

    // Parameters
    let x = tts.compute_spec(&x)?;
    let speaker_cond_src = tts.embed(&rustts::utils::audio::get_wav_files(&speaker_cond_src)?)?;
    let speaker_cond_tgt = tts.embed(&rustts::utils::audio::get_wav_files(&speaker_cond_tgt)?)?;

    // Forward
    let ref_wav_voc = tts.voice_conversion(&x, &speaker_cond_src, &speaker_cond_tgt)?;

    // Save to .wav file
    rustts::utils::audio::save_wav(ref_wav_voc, "output-vc.wav")?;
    Ok(())
}
