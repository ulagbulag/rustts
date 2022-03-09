use std::collections::HashMap;

pub fn text_to_sequence(text: &str, symbols: &HashMap<String, i64>) -> Vec<i64> {
    let sep = *symbols.values().max().unwrap() + 1;
    let seq = itertools::intersperse(
        text.chars()
            .into_iter()
            .map(|e| e.to_ascii_lowercase())
            .filter_map(|e| symbols.get(&e.to_string()))
            .copied(),
        sep,
    );

    [sep].into_iter().chain(seq).chain([sep]).collect()
}

#[test]
fn test_text_to_sequence() {
    let text = "hello world";
    let symbols = [
        '_', '!', '\'', '(', ')', ',', '-', '.', ':', ';', '?', ' ', 'A', 'B', 'C', 'D', 'E', 'F',
        'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
        'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
        'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '¯', '·', 'ß', 'à', 'á', 'â', 'ã', 'ä',
        'æ', 'ç', 'è', 'é', 'ê', 'ë', 'ì', 'í', 'î', 'ï', 'ñ', 'ò', 'ó', 'ô', 'õ', 'ö', 'ù', 'ú',
        'û', 'ü', 'ÿ', 'ā', 'ą', 'ć', 'ē', 'ę', 'ě', 'ī', 'ı', 'ł', 'ń', 'ō', 'ő', 'œ', 'ś', 'ū',
        'ű', 'ź', 'ż', 'ǎ', 'ǐ', 'ǒ', 'ǔ', 'а', 'б', 'в', 'г', 'д', 'е', 'ж', 'з', 'и', 'й', 'к',
        'л', 'м', 'н', 'о', 'п', 'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь',
        'э', 'ю', 'я', 'ё', 'є', 'і', 'ї', 'ґ', '–', '!', '\'', '(', ')', ',', '-', '.', ':', ';',
        '?', ' ',
    ]
    .into_iter()
    .enumerate()
    .map(|(i, v)| (v.to_string(), i as i64))
    .collect();

    assert_eq!(
        text_to_sequence(text, &symbols),
        vec![
            164, 45, 164, 42, 164, 49, 164, 49, 164, 52, 164, 163, 164, 60, 164, 52, 164, 55, 164,
            49, 164, 41, 164,
        ]
    );
}
