use std::{borrow::Cow, ops::Range};

use crate::{
    pinyin::{PinyinData, PinyinNotation},
    unicode::{CharToMonoLowercase, StrToMonoLowercase},
};

pub struct PinyinMatcherBuilder<'a> {
    pattern: String,
    case_insensitive: bool,
    is_pattern_partial: bool,
    pinyin_data: Option<&'a PinyinData>,
    pinyin_notations: PinyinNotation,
    pinyin_case_insensitive: bool,
}

impl<'a> PinyinMatcherBuilder<'a> {
    fn new(pattern: &str) -> Self {
        Self {
            pattern: pattern.to_owned(),
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

    const ORDERED_PINYIN_NOTATIONS: [PinyinNotation; 10] = [
        PinyinNotation::AsciiFirstLetter,
        PinyinNotation::Ascii,
        PinyinNotation::AsciiTone,
        PinyinNotation::Unicode,
        PinyinNotation::DiletterAbc,
        PinyinNotation::DiletterJiajia,
        PinyinNotation::DiletterMicrosoft,
        PinyinNotation::DiletterThunisoft,
        PinyinNotation::DiletterXiaohe,
        PinyinNotation::DiletterZrm,
    ];

    pub fn build(self) -> PinyinMatcher<'a> {
        let pattern_string = self.pattern.clone();
        let pattern_s: &str = pattern_string.as_str();
        let pattern_s: &'static str = unsafe { std::mem::transmute(pattern_s) };

        let pattern_string_lowercase = pattern_string.to_mono_lowercase();
        let pattern_s_lowercase: &str = pattern_string_lowercase.as_str();
        let pattern_s_lowercase: &'static str = unsafe { std::mem::transmute(pattern_s_lowercase) };

        // TODO: If pattern does not contain any pinyin letter, then pinyin_data is not needed.
        PinyinMatcher {
            regex: regex::RegexBuilder::new(&regex::escape(&self.pattern))
                .case_insensitive(self.case_insensitive)
                .build()
                .unwrap(),

            pattern: pattern_string
                .char_indices()
                .zip(pattern_string_lowercase.char_indices())
                .map(|((i, c), (i_lowercase, c_lowercase))| {
                    debug_assert_eq!(i, i_lowercase);
                    PatternChar {
                        c,
                        c_lowercase,
                        s: &pattern_s[i..],
                        s_lowercase: &pattern_s_lowercase[i..],
                    }
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            _pattern_string: pattern_string,
            _pattern_string_lowercase: pattern_string_lowercase,

            case_insensitive: self.case_insensitive,
            is_pattern_partial: self.is_pattern_partial,

            pinyin_data: match self.pinyin_data {
                Some(pinyin_data) => {
                    #[cfg(not(feature = "inmut-data"))]
                    assert!(pinyin_data
                        .inited_notations()
                        .contains(self.pinyin_notations));
                    #[cfg(feature = "inmut-data")]
                    pinyin_data.init_notations(self.pinyin_notations);

                    Cow::Borrowed(pinyin_data)
                }
                None => Cow::Owned(PinyinData::new(self.pinyin_notations)),
            },
            pinyin_notations: {
                let mut notations = Vec::new();
                for notation in Self::ORDERED_PINYIN_NOTATIONS {
                    if self.pinyin_notations.contains(notation) {
                        notations.push(notation);
                    }
                }
                notations.into_boxed_slice()
            },
            pinyin_case_insensitive: self.pinyin_case_insensitive,
        }
    }
}

/// TODO: ASCII-only haystack with non-ASCII pattern optimization
/// TODO: No-pinyin pattern optimization
/// TODO: Match Ascii only after AsciiFirstLetter; get_pinyins_and_for_each
/// TODO: Anchors, `*_at`
/// TODO: UTF-16 and UCS-4
/// TODO: Unicode normalization
/// TODO: No-hanzi haystack optimization (0.2/0.9%)
pub struct PinyinMatcher<'a> {
    /// For ASCII-only haystack optimization.
    regex: regex::Regex,

    pattern: Box<[PatternChar<'a>]>,
    _pattern_string: String,
    _pattern_string_lowercase: String,

    case_insensitive: bool,
    is_pattern_partial: bool,

    pinyin_data: Cow<'a, PinyinData>,
    pinyin_notations: Box<[PinyinNotation]>,
    pinyin_case_insensitive: bool,
}

struct PatternChar<'a> {
    c: char,
    c_lowercase: char,
    s: &'a str,
    s_lowercase: &'a str,
}

pub struct Match {
    start: usize,
    end: usize,
    is_pattern_partial: bool,
}

impl Match {
    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    pub fn is_pattern_partial(&self) -> bool {
        self.is_pattern_partial
    }
}

#[derive(Clone, Copy)]
struct SubMatch {
    len: usize,
    is_pattern_partial: bool,
}

impl SubMatch {
    fn new(len: usize, is_pattern_partial: bool) -> Self {
        Self {
            len,
            is_pattern_partial,
        }
    }
}

impl<'a> PinyinMatcher<'a> {
    pub fn builder(pattern: &str) -> PinyinMatcherBuilder<'a> {
        PinyinMatcherBuilder::new(pattern)
    }

    pub fn find(&self, haystack: &str) -> Option<Match> {
        self.find_with_is_ascii(haystack, haystack.is_ascii())
    }

    fn find_with_is_ascii(&self, haystack: &str, is_ascii: bool) -> Option<Match> {
        if self.pattern.is_empty() {
            return Some(Match {
                start: 0,
                end: 0,
                is_pattern_partial: false,
            });
        }

        if is_ascii {
            return self.regex.find(haystack).map(|m| Match {
                start: m.start(),
                end: m.end(),
                is_pattern_partial: false,
            });
        }

        for (i, _c) in haystack.char_indices() {
            if let Some(submatch) = self.sub_test(&self.pattern, &haystack[i..], 0) {
                return Some(Match {
                    start: i,
                    end: i + submatch.len,
                    is_pattern_partial: submatch.is_pattern_partial,
                });
            }
        }

        None
    }

    pub fn is_match(&self, haystack: &str) -> bool {
        if haystack.is_ascii() {
            return self.regex.is_match(haystack);
        }

        self.find_with_is_ascii(haystack, false).is_some()
    }

    /// ## Returns
    /// - `Match.start()` is guaranteed to be 0.
    /// - If there are multiple possible matches, the longer ones are preferred. But the result is not guaranteed to be the longest one.
    pub fn test(&self, haystack: &str) -> Option<Match> {
        if self.pattern.is_empty() {
            return Some(Match {
                start: 0,
                end: 0,
                is_pattern_partial: false,
            });
        }

        if haystack.is_ascii() {
            // TODO: Use regex-automata's anchored searches?
            return match self.regex.find(haystack) {
                Some(m) => match m.start() {
                    0 => Some(Match {
                        start: 0,
                        end: m.end(),
                        is_pattern_partial: false,
                    }),
                    _ => None,
                },
                None => None,
            };
        }

        self.sub_test(&self.pattern, haystack, 0)
            .map(|submatch| Match {
                start: 0,
                end: submatch.len,
                is_pattern_partial: submatch.is_pattern_partial,
            })
    }

    /// ## Arguments
    /// - `pattern`: Not empty.
    /// - `haystack`
    /// - `matched_len`: For tail-call optimization.
    fn sub_test(
        &self,
        pattern: &[PatternChar],
        haystack: &str,
        matched_len: usize,
    ) -> Option<SubMatch> {
        debug_assert!(!pattern.is_empty());

        let (haystack_c, haystack_next) = {
            let mut chars = haystack.chars();
            match chars.next() {
                Some(c) => (c, chars.as_str()),
                None => return None,
            }
        };

        let (pattern_c, pattern_next) = pattern.split_first().unwrap();

        if match self.case_insensitive {
            true => haystack_c.to_mono_lowercase() == pattern_c.c_lowercase,
            false => haystack_c == pattern_c.c,
        } {
            // If haystack_c == pattern_c, then it is impossible that pattern_c is a pinyin letter and haystack_c is a hanzi.
            let matched_len = matched_len + haystack_c.len_utf8();
            return if pattern_next.is_empty() {
                Some(SubMatch::new(matched_len, false))
            } else {
                self.sub_test(pattern_next, haystack_next, matched_len)
            };
        }

        for pinyin in self.pinyin_data.get_pinyins(haystack_c) {
            for &notation in self.pinyin_notations.iter() {
                let pinyin = pinyin.notation(notation).unwrap();
                if let Some(submatch) = self.sub_test_pinyin(pattern, haystack, matched_len, pinyin)
                {
                    return Some(submatch);
                }
            }
        }

        None
    }

    /// ## Arguments
    /// - `pattern`: Not empty.
    /// - `haystack`
    /// - `matched_len`: For tail-call optimization.
    fn sub_test_pinyin(
        &self,
        pattern: &[PatternChar],
        haystack: &str,
        matched_len: usize,
        pinyin: &str,
    ) -> Option<SubMatch> {
        debug_assert!(!pattern.is_empty());
        debug_assert_eq!(pinyin, pinyin.to_lowercase());

        let pattern_s = match self.pinyin_case_insensitive {
            true => pattern[0].s_lowercase,
            false => pattern[0].s,
        };

        let haystack_c_len = haystack.chars().next().unwrap().len_utf8();
        let matched_len = matched_len + haystack_c_len;

        if pattern_s.len() < pinyin.len() {
            if self.is_pattern_partial && pinyin.starts_with(pattern_s) {
                return Some(SubMatch::new(matched_len, true));
            }
        } else if pattern_s.starts_with(pinyin) {
            if pattern_s.len() == pinyin.len() {
                return Some(SubMatch::new(matched_len, false));
            }

            if let Some(submatch) = self.sub_test(
                &pattern[pinyin.chars().count()..],
                &haystack[haystack_c_len..],
                matched_len,
            ) {
                return Some(submatch);
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_match(m: Option<Match>, expected: Option<(usize, usize)>) {
        assert_eq!(m.map(|m| (m.start(), m.len())), expected);
    }

    #[test]
    fn ordered_pinyin_notations() {
        assert_eq!(
            PinyinNotation::all().iter().count(),
            PinyinMatcherBuilder::ORDERED_PINYIN_NOTATIONS.len()
        )
    }

    #[test]
    fn test() {
        let matcher = PinyinMatcher::builder("xing")
            .pinyin_notations(PinyinNotation::Ascii)
            .build();
        assert_match(matcher.test(""), None);
        assert_match(matcher.test("xing"), Some((0, 4)));
        assert_match(matcher.test("XiNG"), Some((0, 4)));
        assert_match(matcher.test("行"), Some((0, 3)));

        let matcher = PinyinMatcher::builder("ke")
            .pinyin_notations(PinyinNotation::Ascii)
            .build();
        assert_match(matcher.test("ke"), Some((0, 2)));
        assert_match(matcher.test("科"), Some((0, 3)));
        assert_match(matcher.test("k鹅"), Some((0, 4)));
        assert_match(matcher.test("凯尔"), None);

        let matcher = PinyinMatcher::builder("")
            .pinyin_notations(PinyinNotation::Ascii)
            .build();
        assert_match(matcher.test(""), Some((0, 0)));
        assert_match(matcher.test("abc"), Some((0, 0)));

        let matcher = PinyinMatcher::builder("ke")
            .pinyin_notations(PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter)
            .build();
        assert_match(matcher.test("ke"), Some((0, 2)));
        assert_match(matcher.test("科"), Some((0, 3)));
        assert_match(matcher.test("k鹅"), Some((0, 4)));
        assert_match(matcher.test("凯尔"), Some((0, 6)));
        // AsciiFirstLetter is preferred
        assert_match(matcher.test("柯尔"), Some((0, 6)));
    }

    #[test]
    fn test_case_insensitive() {
        let matcher = PinyinMatcher::builder("xing")
            .case_insensitive(false)
            .pinyin_case_insensitive(false)
            .pinyin_notations(PinyinNotation::Ascii)
            .build();
        assert_match(matcher.test("xing"), Some((0, 4)));
        assert_match(matcher.test("XiNG"), None);
        assert_match(matcher.test("行"), Some((0, 3)));

        let matcher = PinyinMatcher::builder("XING")
            .case_insensitive(true)
            .pinyin_case_insensitive(false)
            .pinyin_notations(PinyinNotation::Ascii)
            .build();
        assert_match(matcher.test("xing"), Some((0, 4)));
        assert_match(matcher.test("XiNG"), Some((0, 4)));
        assert_match(matcher.test("行"), None);

        let matcher = PinyinMatcher::builder("XING")
            .case_insensitive(true)
            .pinyin_case_insensitive(true)
            .pinyin_notations(PinyinNotation::Ascii)
            .build();
        assert_match(matcher.test("xing"), Some((0, 4)));
        assert_match(matcher.test("XiNG"), Some((0, 4)));
        assert_match(matcher.test("行"), Some((0, 3)));

        let matcher = PinyinMatcher::builder("XiNG")
            .case_insensitive(false)
            .pinyin_case_insensitive(true)
            .pinyin_notations(PinyinNotation::Ascii)
            .build();
        assert_match(matcher.test("xing"), None);
        assert_match(matcher.test("XiNG"), Some((0, 4)));
        assert_match(matcher.test("行"), Some((0, 3)));
    }

    #[test]
    fn find() {
        let matcher = PinyinMatcher::builder("xing")
            .pinyin_notations(PinyinNotation::Ascii)
            .build();
        assert_match(matcher.find(""), None);
        assert_match(matcher.find("buxing"), Some((2, 4)));
        assert_match(matcher.find("BuXiNG"), Some((2, 4)));
        assert_match(matcher.find("不行"), Some((3, 3)));

        let matcher = PinyinMatcher::builder("")
            .pinyin_notations(PinyinNotation::Ascii)
            .build();
        assert_match(matcher.find(""), Some((0, 0)));
        assert_match(matcher.find("abc"), Some((0, 0)));
    }
}
