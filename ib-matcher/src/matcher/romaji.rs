use std::borrow::Cow;

use bon::Builder;
use ib_romaji::HepburnRomanizer;

#[derive(Builder, Clone)]
pub struct RomajiMatchConfig<'a> {
    /// Default: `new()` on [`RomajiMatchConfigBuilder::build()`]
    #[builder(default = Cow::Owned(HepburnRomanizer::new()))]
    #[builder(with = |romanizer: &'a HepburnRomanizer| Cow::Borrowed(romanizer))]
    pub(crate) romanizer: Cow<'a, HepburnRomanizer>,

    #[builder(default = false)]
    pub(crate) case_insensitive: bool,
}

impl Default for RomajiMatchConfig<'_> {
    /// Use [`RomajiMatchConfigBuilder`] for more options.
    fn default() -> Self {
        Self::builder().build()
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_match, matcher::IbMatcher};

    use super::*;

    #[test]
    fn romaji() {
        let matcher = IbMatcher::builder("ohayo")
            .romaji(RomajiMatchConfig::default())
            .build();
        assert_match!(matcher.find("おはよう"), Some((0, 9)));

        let matcher = IbMatcher::builder("jojo")
            .romaji(RomajiMatchConfig::default())
            .build();
        assert_match!(matcher.find("おはよジョジョ"), Some((9, 12)));
    }
}
