use anyhow::Result;

fn main() -> Result<()> {
    let model_path = "./assets/vits.pt";
    let speaker_encoder_path = "./assets/speaker_encoder.pt";
    let tts = rustts::TTS::try_default(model_path, speaker_encoder_path)?;

    let (driving_spec, y_lengths) = tts.compute_spec("../recording.wav")?;

    let target_emb = tts.embed(&[
        "../target_0.wav",
        "../target_1.wav",
        "../target_2.wav",
        "../target_3.wav",
        "../target_4.wav",
        "../target_5.wav",
        "../target_6.wav",
        "../target_7.wav",
        "../target_8.wav",
    ])?;

    let driving_emb = tts.embed(&[
        "../driving_ref_1.wav",
        "../driving_ref_3.wav",
        "../driving_ref_4.wav",
        "../driving_ref_5.wav",
        "../driving_ref_6.wav",
        "../driving_ref_7.wav",
        "../driving_ref_8.wav",
        "../driving_ref_9.wav",
        "../driving_ref_10.wav",
        "../driving_ref_11.wav",
        "../driving_ref_12.wav",
    ])?;

    let ref_wav_voc = tts.voice_conversion(&driving_spec, &y_lengths, &driving_emb, &target_emb)?;

    rustts::audio::save_wav(ref_wav_voc, "output.wav")?;
    Ok(())
}
