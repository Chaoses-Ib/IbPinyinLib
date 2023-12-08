pub trait CharToMonoLowercase {
    /// The only multi-char lowercase mapping is 'İ' -> "i\u{307}", we just ignore the '\u{307}'.
    fn to_mono_lowercase(self) -> char;
}

impl CharToMonoLowercase for char {
    fn to_mono_lowercase(self) -> char {
        self.to_lowercase().next().unwrap()
    }
}

pub trait StrToMonoLowercase {
    fn to_mono_lowercase(&self) -> String;
}

impl StrToMonoLowercase for str {
    fn to_mono_lowercase(&self) -> String {
        self.chars().map(|c| c.to_mono_lowercase()).collect()
    }
}
