use ib_matcher::{
    matcher::{analyze, encoding::EncodedStr, IbMatcher, PinyinMatchConfig},
    pinyin::{PinyinData, PinyinNotation},
};

pub use ib_matcher::matcher::{encoding, Match};

/// See [`PinyinMatcher`].
///
/// ## Design
/// API follows [`regex::RegexBuilder`](https://docs.rs/regex/latest/regex/struct.RegexBuilder.html).
pub struct PinyinMatcherBuilder<'a, HaystackStr = str>
where
    HaystackStr: EncodedStr + ?Sized,
{
    pattern: &'a HaystackStr,
    analyze: bool,
    analyze_config: Option<analyze::PatternAnalyzeConfig>,
    case_insensitive: bool,
    is_pattern_partial: bool,
    pinyin_data: Option<&'a PinyinData>,
    pinyin_notations: PinyinNotation,
    pinyin_case_insensitive: bool,
}

impl<'a, HaystackStr> PinyinMatcherBuilder<'a, HaystackStr>
where
    HaystackStr: EncodedStr + ?Sized,
{
    /// Use [`PinyinMatcher::builder()`] instead.
    fn new(pattern: &'a HaystackStr) -> Self {
        Self {
            pattern,
            analyze: false,
            analyze_config: None,
            case_insensitive: true,
            is_pattern_partial: false,
            pinyin_data: None,
            pinyin_notations: PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter,
            pinyin_case_insensitive: false,
        }
    }

    /// Default: `true`
    ///
    /// The case insensitivity of pinyin is controlled by `pinyin_case_insensitive`.
    pub fn case_insensitive(mut self, case_insensitive: bool) -> Self {
        self.case_insensitive = case_insensitive;
        self
    }

    /// Default: `false`
    pub fn is_pattern_partial(mut self, is_pattern_partial: bool) -> Self {
        self.is_pattern_partial = is_pattern_partial;
        self
    }

    /// Default: `new()` on `build()`
    ///
    /// ## Arguments
    /// - `pinyin_data`: Must be inited with required notations if `inmut-data` feature is not enabled.
    pub fn pinyin_data(mut self, pinyin_data: &'a PinyinData) -> Self {
        self.pinyin_data = Some(pinyin_data);
        self
    }

    /// Default: `PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter`
    pub fn pinyin_notations(mut self, pinyin_notations: PinyinNotation) -> Self {
        self.pinyin_notations = pinyin_notations;
        self
    }

    /// Default: `false`
    pub fn pinyin_case_insensitive(mut self, pinyin_case_insensitive: bool) -> Self {
        self.pinyin_case_insensitive = pinyin_case_insensitive;
        self
    }

    pub fn analyze_config(mut self, config: analyze::PatternAnalyzeConfig) -> Self {
        self.analyze_config = Some(config);
        self
    }

    /// For more advanced control over the analysis, use [`PinyinMatcherBuilder::analyze_config`].
    pub fn analyze(mut self, analyze: bool) -> Self {
        self.analyze = analyze;
        self
    }

    pub fn build(self) -> PinyinMatcher<'a, HaystackStr> {
        PinyinMatcher {
            matcher: IbMatcher::builder(self.pattern)
                .analyze(self.analyze)
                .maybe_analyze_config(self.analyze_config)
                .case_insensitive(self.case_insensitive)
                .is_pattern_partial(self.is_pattern_partial)
                .pinyin(
                    PinyinMatchConfig::builder(self.pinyin_notations)
                        .maybe_data(self.pinyin_data)
                        .case_insensitive(self.pinyin_case_insensitive)
                        .build(),
                )
                .build(),
        }
    }
}

pub struct PinyinMatcher<'a, HaystackStr = str>
where
    HaystackStr: EncodedStr + ?Sized,
{
    matcher: IbMatcher<'a, HaystackStr>,
}

impl<'a, HaystackStr> PinyinMatcher<'a, HaystackStr>
where
    HaystackStr: EncodedStr + ?Sized,
{
    pub fn builder(pattern: &'a HaystackStr) -> PinyinMatcherBuilder<'a, HaystackStr> {
        PinyinMatcherBuilder::new(pattern)
    }

    pub fn find(&self, haystack: &HaystackStr) -> Option<Match> {
        self.matcher.find(haystack)
    }

    pub fn is_match(&self, haystack: &HaystackStr) -> bool {
        self.matcher.is_match(haystack)
    }

    /// ## Returns
    /// - `Match.start()` is guaranteed to be 0.
    /// - If there are multiple possible matches, the longer ones are preferred. But the result is not guaranteed to be the longest one.
    pub fn test(&self, haystack: &HaystackStr) -> Option<Match> {
        self.matcher.test(haystack)
    }
}
