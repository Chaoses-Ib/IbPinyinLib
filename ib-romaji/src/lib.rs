use aho_corasick::{AhoCorasick, Anchored, Input, MatchKind, StartKind};

mod kana;

/// https://en.wikipedia.org/wiki/Hepburn_romanization
///
/// TODO: Kanji
#[derive(Clone)]
pub struct HepburnRomanizer {
    ac: AhoCorasick,
}

impl HepburnRomanizer {
    pub fn new() -> Self {
        Self {
            ac: AhoCorasick::builder()
                .start_kind(StartKind::Anchored)
                .match_kind(MatchKind::LeftmostLongest)
                .build(kana::HEPBURN_KANAS)
                .unwrap(),
        }
    }

    /// ```
    /// use ib_romaji::HepburnRomanizer;
    ///
    /// assert_eq!(HepburnRomanizer::new().romanize("あ"), Some((3, "a")));
    /// ```
    /// TODO: Iter
    pub fn romanize<S: ?Sized + AsRef<[u8]>>(&self, s: &S) -> Option<(usize, &'static str)> {
        let m = self.ac.find(Input::new(s).anchored(Anchored::Yes))?;
        Some((m.len(), kana::HEPBURN_ROMAJIS[m.pattern()]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hepburn() {
        let data = HepburnRomanizer::new();
        assert_eq!(data.romanize("は"), Some((3, "ha")));
        assert_eq!(data.romanize("ハハハ"), Some((3, "ha")));
        assert_eq!(data.romanize("ジョジョ"), Some((6, "jo")));
        assert_eq!(data.romanize("って"), Some((6, "tte")));
        assert_eq!(data.romanize("日は"), None);
    }
}
