use tch::Tensor;

#[derive(Clone, Debug)]
pub struct Config {
    pub audio: super::audio::AudioConfig,
    pub symbols: Vec<char>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            audio: Default::default(),
            symbols: vec![
                '_', '!', '\'', '(', ')', ',', '-', '.', ':', ';', '?', ' ', 'A', 'B', 'C', 'D',
                'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
                'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
                'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
                '¯', '·', 'ß', 'à', 'á', 'â', 'ã', 'ä', 'æ', 'ç', 'è', 'é', 'ê', 'ë', 'ì', 'í',
                'î', 'ï', 'ñ', 'ò', 'ó', 'ô', 'õ', 'ö', 'ù', 'ú', 'û', 'ü', 'ÿ', 'ā', 'ą', 'ć',
                'ē', 'ę', 'ě', 'ī', 'ı', 'ł', 'ń', 'ō', 'ő', 'œ', 'ś', 'ū', 'ű', 'ź', 'ż', 'ǎ',
                'ǐ', 'ǒ', 'ǔ', 'а', 'б', 'в', 'г', 'д', 'е', 'ж', 'з', 'и', 'й', 'к', 'л', 'м',
                'н', 'о', 'п', 'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь',
                'э', 'ю', 'я', 'ё', 'є', 'і', 'ї', 'ґ', '–', '!', '\'', '(', ')', ',', '-', '.',
                ':', ';', '?', ' ',
            ],
        }
    }
}

impl Config {
    pub fn text_to_sequence(&self, text: &str) -> Tensor {
        let symbols = self
            .symbols
            .iter()
            .copied()
            .enumerate()
            .map(|(i, v)| (v.to_string(), i as i64))
            .collect();

        Tensor::of_slice(&super::text::text_to_sequence(text, &symbols))
    }
}
