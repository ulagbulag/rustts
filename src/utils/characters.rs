#[derive(Clone, Debug)]
pub struct Characters {
    pub pad: String,
    pub eos: String,
    pub bos: String,
    pub characters: String,
    pub punctuations: String,
    pub phonemes: String,
    pub unique: bool,
}

impl Default for Characters {
    fn default() -> Self {
        Self::with_en_us()
    }
}

impl Characters {
    pub fn with_en_us() -> Self {
        Self {
            pad: "_".to_string(),
            eos: "&".to_string(),
            bos: "*".to_string(),
            characters: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz\u{00af}\u{00b7}\u{00df}\u{00e0}\u{00e1}\u{00e2}\u{00e3}\u{00e4}\u{00e6}\u{00e7}\u{00e8}\u{00e9}\u{00ea}\u{00eb}\u{00ec}\u{00ed}\u{00ee}\u{00ef}\u{00f1}\u{00f2}\u{00f3}\u{00f4}\u{00f5}\u{00f6}\u{00f9}\u{00fa}\u{00fb}\u{00fc}\u{00ff}\u{0101}\u{0105}\u{0107}\u{0113}\u{0119}\u{011b}\u{012b}\u{0131}\u{0142}\u{0144}\u{014d}\u{0151}\u{0153}\u{015b}\u{016b}\u{0171}\u{017a}\u{017c}\u{01ce}\u{01d0}\u{01d2}\u{01d4}\u{0430}\u{0431}\u{0432}\u{0433}\u{0434}\u{0435}\u{0436}\u{0437}\u{0438}\u{0439}\u{043a}\u{043b}\u{043c}\u{043d}\u{043e}\u{043f}\u{0440}\u{0441}\u{0442}\u{0443}\u{0444}\u{0445}\u{0446}\u{0447}\u{0448}\u{0449}\u{044a}\u{044b}\u{044c}\u{044d}\u{044e}\u{044f}\u{0451}\u{0454}\u{0456}\u{0457}\u{0491}\u{2013}!'(),-.:;? ".to_string(),
            punctuations: "!'(),-.:;? ".to_string(),
            phonemes: "iy\u{0268}\u{0289}\u{026f}u\u{026a}\u{028f}\u{028ae}\u{00f8}\u{0258}\u{0259}\u{0275}\u{0264}o\u{025b}\u{0153}\u{025c}\u{025e}\u{028c}\u{0254}\u{00e6}\u{0250a}\u{0276}\u{0251}\u{0252}\u{1d7b}\u{0298}\u{0253}\u{01c0}\u{0257}\u{01c3}\u{0284}\u{01c2}\u{0260}\u{01c1}\u{029b}pbtd\u{0288}\u{0256}c\u{025f}k\u{0261}q\u{0262}\u{0294}\u{0274}\u{014b}\u{0272}\u{0273}n\u{0271}m\u{0299}r\u{0280}\u{2c71}\u{027e}\u{027d}\u{0278}\u{03b2}fv\u{03b8}\u{00f0}sz\u{0283}\u{0292}\u{0282}\u{0290}\u{00e7}\u{029d}x\u{0263}\u{03c7}\u{0281}\u{0127}\u{0295}h\u{0266}\u{026c}\u{026e}\u{028b}\u{0279}\u{027b}j\u{0270}l\u{026d}\u{028e}\u{029f}\u{02c8}\u{02cc}\u{02d0}\u{02d1}\u{028d}w\u{0265}\u{029c}\u{02a2}\u{02a1}\u{0255}\u{0291}\u{027a}\u{0267}\u{025a}\u{02de}\u{026b}'\u{0303}' ".to_string(),
            unique: true,
        }
    }
}
