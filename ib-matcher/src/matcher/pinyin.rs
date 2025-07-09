use std::borrow::Cow;

use bon::Builder;

use crate::pinyin::{PinyinData, PinyinNotation};

#[derive(Builder, Clone)]
pub struct PinyinMatchConfig<'a> {
    #[builder(start_fn)]
    pub(crate) notations: PinyinNotation,

    /// Default: `new()` on [`PinyinMatchConfigBuilder::build()`]
    ///
    /// Must be inited with required notations if `inmut-data` feature is not enabled.
    #[builder(default = Cow::Owned(PinyinData::new(notations)))]
    #[builder(with = |data: &'a PinyinData| Cow::Borrowed(data))]
    pub(crate) data: Cow<'a, PinyinData>,

    #[builder(default = false)]
    pub(crate) case_insensitive: bool,
}

impl<'a> PinyinMatchConfig<'a> {
    /// Use [`PinyinMatchConfigBuilder`] for more options.
    pub fn notations(notations: PinyinNotation) -> Self {
        Self::builder(notations).build()
    }
}
