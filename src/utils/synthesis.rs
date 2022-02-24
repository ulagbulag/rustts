use super::config::Config;

pub fn test_to_seq(text: &str, config: &Config, custom_symbols: Option<&[&str]>) {
    let text_cleaner = vec![config.text_cleaner.as_str()];

    // text ot phonemes to sequence vector
    if config.use_phonemes {
        // TODO: to be implemented
        todo!()
    } else {
        let seq = super::text::text_to_sequence(
            text,
            &text_cleaner,
            custom_symbols,
            &config.characters,
            config.add_blank,
        );
    }
}
