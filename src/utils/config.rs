#[derive(Clone, Debug)]
pub struct Config {
    pub audio: super::audio::AudioConfig,
    pub characters: super::characters::Characters,

    pub add_blank: bool,
    pub use_phonemes: bool,
    pub text_cleaner: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            audio: Default::default(),
            characters: Default::default(),

            add_blank: true,
            use_phonemes: false,
            text_cleaner: "multilingual_cleaners".to_string(),
        }
    }
}
