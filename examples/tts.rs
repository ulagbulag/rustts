use anyhow::Result;
use rustts::tts::SynthesisOptions;

fn main() -> Result<()> {
    // Load a Model
    let model_path = "./assets/vits";
    let speaker_encoder_path = "./assets/speaker_encoder.pt";
    let tts = rustts::TTS::try_default(model_path, speaker_encoder_path)?;

    // Command-line Arguments
    let text = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "Hello world.".to_string());
    let speaker_emb = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "./assets/samples/speaker_man_korean/".to_string());

    // Parameters
    let text = text;
    let speaker_emb = tts.embed(&rustts::utils::audio::get_wav_files(&speaker_emb)?)?;
    let options = SynthesisOptions {
        length_scale: 1.0,
        ..Default::default()
    };

    // Forward
    let ref_wav_voc = tts.synthesis(&text, &speaker_emb, &options)?;

    // Save to .wav file
    rustts::utils::audio::save_wav(ref_wav_voc, "output-tts.wav")?;
    Ok(())
}
