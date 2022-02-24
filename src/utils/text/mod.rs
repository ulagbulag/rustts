use super::characters::Characters;

pub fn text_to_sequence(
    text: &str,
    cleaner_names: &[&str],
    custom_symbols: Option<&[&str]>,
    tp: &Characters,
    add_blank: bool,
) {
    let symbols = custom_symbols.unwrap_or_else(|| {
        // TODO: to be implemented
        todo!()
    });

    let symbol_to_id = symbols.iter().enumerate().map(|(i, s)| (s, i)).collect();

    let sequence = vec![];

    let mut text = text;
    while text {}
}
