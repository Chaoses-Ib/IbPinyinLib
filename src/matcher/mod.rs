use std::{borrow::Cow, marker::PhantomData, ops::Range};

use crate::pinyin::{PinyinData, PinyinNotation};

mod unicode;
use unicode::{CharToMonoLowercase, StrToMonoLowercase};

pub mod encoding;
use encoding::EncodedStr;

mod regex_utils;

pub struct PinyinMatcherBuilder<'a, HaystackStr = str>
where
    HaystackStr: EncodedStr + ?Sized,
{
    pattern: String,
    pattern_bytes: Vec<u8>,
    case_insensitive: bool,
    is_pattern_partial: bool,
    pinyin_data: Option<&'a PinyinData>,
    pinyin_notations: PinyinNotation,
    pinyin_case_insensitive: bool,

    _haystack_str: PhantomData<HaystackStr>,
}

impl<'a, HaystackStr> PinyinMatcherBuilder<'a, HaystackStr>
where
    HaystackStr: EncodedStr + ?Sized,
{
    fn new(pattern: &HaystackStr) -> Self {
        Self {
            pattern: pattern.char_index_strs().map(|(_, c, _)| c).collect(),
            pattern_bytes: pattern.as_bytes().to_owned(),
            case_insensitive: true,
            is_pattern_partial: false,
            pinyin_data: None,
            pinyin_notations: PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter,
            pinyin_case_insensitive: false,

            _haystack_str: PhantomData,
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

    pub fn build(self) -> PinyinMatcher<'a, HaystackStr> {
        let pattern_string = self.pattern;
        let pattern_s: &str = pattern_string.as_str();
        let pattern_s: &'static str = unsafe { std::mem::transmute(pattern_s) };

        let pattern_string_lowercase = pattern_string.to_mono_lowercase();
        let pattern_s_lowercase: &str = pattern_string_lowercase.as_str();
        let pattern_s_lowercase: &'static str = unsafe { std::mem::transmute(pattern_s_lowercase) };

        let (notations_prefix_group, unprefixable_pinyin_notations) = match self
            .pinyin_notations
            .intersection(
                PinyinNotation::AsciiFirstLetter
                    | PinyinNotation::Ascii
                    | PinyinNotation::AsciiTone,
            )
            .bits()
            .count_ones()
        {
            count if count > 1 => {
                let mut notations = Vec::with_capacity(count as usize);
                if self
                    .pinyin_notations
                    .contains(PinyinNotation::AsciiFirstLetter)
                {
                    notations.push(PinyinNotation::AsciiFirstLetter);
                }
                if self.pinyin_notations.contains(PinyinNotation::Ascii) {
                    notations.push(PinyinNotation::Ascii);
                }
                if self.pinyin_notations.contains(PinyinNotation::AsciiTone) {
                    notations.push(PinyinNotation::AsciiTone);
                }
                (
                    notations,
                    self.pinyin_notations.difference(
                        PinyinNotation::AsciiFirstLetter
                            | PinyinNotation::Ascii
                            | PinyinNotation::AsciiTone,
                    ),
                )
            }
            _ => (Vec::new(), self.pinyin_notations),
        };
        let mut notations =
            Vec::with_capacity(unprefixable_pinyin_notations.bits().count_ones() as usize);
        for notation in Self::ORDERED_PINYIN_NOTATIONS {
            if unprefixable_pinyin_notations.contains(notation) {
                notations.push(notation);
            }
        }

        let pattern = pattern_string
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
            .into_boxed_slice();

        // TODO: A better lower bound
        let min_haystack_chars = {
            match self.pinyin_notations.max_len() {
                Some(max_len) => {
                    // - Ascii: "shuang" / 6 = 1, "a" / 6 = 1
                    pattern.len().div_ceil(max_len)
                }
                None => {
                    // If case_insensitive, pattern length in bytes may be shorter than the matched haystack (or not?), so we use char count only
                    pattern.len()
                }
            }
        };

        // TODO: If pattern does not contain any pinyin letter, then pinyin_data is not needed.
        PinyinMatcher {
            // ASCII-only haystack optimization
            regex: match self.pattern_bytes.is_ascii() {
                true => Some(
                    regex::bytes::RegexBuilder::new(&regex_utils::escape_bytes(
                        &self.pattern_bytes,
                    ))
                    .unicode(false)
                    .case_insensitive(self.case_insensitive)
                    .build()
                    .unwrap(),
                ),
                // ASCII-only haystack with non-ASCII pattern optimization
                false => None,
            },

            pattern,
            _pattern_string: pattern_string,
            _pattern_string_lowercase: pattern_string_lowercase,

            min_haystack_chars,

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
            pinyin_notations_prefix_group: notations_prefix_group.into_boxed_slice(),
            pinyin_notations: notations.into_boxed_slice(),
            pinyin_case_insensitive: self.pinyin_case_insensitive,

            _haystack_str: PhantomData,
        }
    }
}

/// TODO: No-pinyin pattern optimization
/// TODO: Anchors, `*_at`
/// TODO: Unicode normalization
/// TODO: No-hanzi haystack optimization (0.2/0.9%)
/// TODO: If pattern doesn't contain `.`, only match before `.` in the haystack
pub struct PinyinMatcher<'a, HaystackStr = str>
where
    HaystackStr: EncodedStr + ?Sized,
{
    /// For ASCII-only haystack optimization.
    regex: Option<regex::bytes::Regex>,

    pattern: Box<[PatternChar<'a>]>,
    _pattern_string: String,
    _pattern_string_lowercase: String,

    min_haystack_chars: usize,

    case_insensitive: bool,
    is_pattern_partial: bool,

    pinyin_data: Cow<'a, PinyinData>,
    pinyin_notations_prefix_group: Box<[PinyinNotation]>,
    pinyin_notations: Box<[PinyinNotation]>,
    pinyin_case_insensitive: bool,

    _haystack_str: PhantomData<HaystackStr>,
}

struct PatternChar<'a> {
    c: char,
    c_lowercase: char,
    s: &'a str,
    s_lowercase: &'a str,
}

#[derive(Clone, Debug)]
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

impl<'a, HaystackStr> PinyinMatcher<'a, HaystackStr>
where
    HaystackStr: EncodedStr + ?Sized,
{
    pub fn builder(pattern: &HaystackStr) -> PinyinMatcherBuilder<'a, HaystackStr> {
        PinyinMatcherBuilder::new(pattern)
    }

    pub fn find(&self, haystack: &HaystackStr) -> Option<Match> {
        self.find_with_is_ascii(haystack, haystack.is_ascii())
    }

    fn find_with_is_ascii(&self, haystack: &HaystackStr, is_ascii: bool) -> Option<Match> {
        if self.pattern.is_empty() {
            return Some(Match {
                start: 0,
                end: 0,
                is_pattern_partial: false,
            });
        }

        if is_ascii {
            return self
                .regex
                .as_ref()
                .map(|regex| {
                    regex.find(haystack.as_bytes()).map(|m| Match {
                        start: m.start() / HaystackStr::ELEMENT_LEN_BYTE,
                        end: m.end() / HaystackStr::ELEMENT_LEN_BYTE,
                        is_pattern_partial: false,
                    })
                })
                .flatten();
        }

        for (i, _c, str) in haystack.char_index_strs() {
            if self.is_haystack_too_short(str) {
                break;
            }
            if let Some(submatch) = self.sub_test(&self.pattern, str, 0) {
                return Some(Match {
                    start: i,
                    end: i + submatch.len,
                    is_pattern_partial: submatch.is_pattern_partial,
                });
            }
        }

        None
    }

    pub fn is_match(&self, haystack: &HaystackStr) -> bool {
        if haystack.is_ascii() {
            return self
                .regex
                .as_ref()
                .map(|regex| regex.is_match(haystack.as_bytes()))
                .unwrap_or(false);
        }

        self.find_with_is_ascii(haystack, false).is_some()
    }

    /// ## Returns
    /// - `Match.start()` is guaranteed to be 0.
    /// - If there are multiple possible matches, the longer ones are preferred. But the result is not guaranteed to be the longest one.
    pub fn test(&self, haystack: &HaystackStr) -> Option<Match> {
        if self.is_haystack_too_short(haystack) {
            return None;
        } else {
            if self.pattern.is_empty() {
                return Some(Match {
                    start: 0,
                    end: 0,
                    is_pattern_partial: false,
                });
            }
        }

        if haystack.is_ascii() {
            // TODO: Use regex-automata's anchored searches?
            return self
                .regex
                .as_ref()
                .map(|regex| {
                    regex
                        .find(haystack.as_bytes())
                        .filter(|m| m.start() == 0)
                        .map(|m| Match {
                            start: 0,
                            end: m.end() / HaystackStr::ELEMENT_LEN_BYTE,
                            is_pattern_partial: false,
                        })
                })
                .flatten();
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
        haystack: &HaystackStr,
        matched_len: usize,
    ) -> Option<SubMatch> {
        debug_assert!(!pattern.is_empty());

        // if Self::is_haystack_too_short_with_pattern(pattern, haystack) {
        //     return None;
        // }

        let (haystack_c, haystack_c_len, haystack_next) = {
            match haystack.char_len_next_strs().next() {
                Some(v) => v,
                None => {
                    return None;

                    // // pattern is not empty, so haystack must not be empty too.
                    // unsafe { unreachable_unchecked() }
                }
            }
        };
        let matched_len = matched_len + haystack_c_len;

        let (pattern_c, pattern_next) = pattern.split_first().unwrap();

        if match self.case_insensitive {
            true => haystack_c.to_mono_lowercase() == pattern_c.c_lowercase,
            false => haystack_c == pattern_c.c,
        } {
            // If haystack_c == pattern_c, then it is impossible that pattern_c is a pinyin letter and haystack_c is a hanzi.
            return if pattern_next.is_empty() {
                Some(SubMatch::new(matched_len, false))
            } else {
                self.sub_test(pattern_next, haystack_next, matched_len)
            };
        }

        // for pinyin in self.pinyin_data.get_pinyins(haystack_c) {
        //     for &notation in self.pinyin_notations_prefix_group.iter() {
        //         let pinyin = pinyin.notation(notation).unwrap();
        //         match self.sub_test_pinyin(pattern, haystack_next, matched_len, pinyin) {
        //             (true, Some(submatch)) => return Some(submatch),
        //             (true, None) => (),
        //             (false, None) => break,
        //             (false, Some(_)) => unreachable!(),
        //         }
        //     }
        //     for &notation in self.pinyin_notations.iter() {
        //         let pinyin = pinyin.notation(notation).unwrap();
        //         match self.sub_test_pinyin(pattern, haystack_next, matched_len, pinyin) {
        //             (true, Some(submatch)) => return Some(submatch),
        //             (true, None) => (),
        //             (false, None) => (),
        //             (false, Some(_)) => unreachable!(),
        //         }
        //     }
        // }
        // None

        // Reduce total time by 45~65% compared to using `get_pinyins()`
        self.pinyin_data
            .get_pinyins_and_try_for_each(haystack_c, |pinyin| {
                for &notation in self.pinyin_notations_prefix_group.iter() {
                    let pinyin = pinyin.notation(notation).unwrap();
                    match self.sub_test_pinyin(pattern, haystack_next, matched_len, pinyin) {
                        (true, Some(submatch)) => return Some(submatch),
                        (true, None) => (),
                        (false, None) => break,
                        (false, Some(_)) => unreachable!(),
                    }
                }
                for &notation in self.pinyin_notations.iter() {
                    let pinyin = pinyin.notation(notation).unwrap();
                    match self.sub_test_pinyin(pattern, haystack_next, matched_len, pinyin) {
                        (true, Some(submatch)) => return Some(submatch),
                        (true, None) => (),
                        (false, None) => (),
                        (false, Some(_)) => unreachable!(),
                    }
                }
                None
            })
    }

    /// ## Arguments
    /// - `pattern`: Not empty.
    /// - `haystack`
    /// - `matched_len`: For tail-call optimization.
    ///
    /// ## Returns
    /// (pinyin_matched, submatch)
    fn sub_test_pinyin(
        &self,
        pattern: &[PatternChar],
        haystack_next: &HaystackStr,
        matched_len_next: usize,
        pinyin: &str,
    ) -> (bool, Option<SubMatch>) {
        debug_assert!(!pattern.is_empty());
        debug_assert_eq!(pinyin, pinyin.to_lowercase());

        let pattern_s = match self.pinyin_case_insensitive {
            true => pattern[0].s_lowercase,
            false => pattern[0].s,
        };

        if pattern_s.len() < pinyin.len() {
            if self.is_pattern_partial && pinyin.starts_with(pattern_s) {
                return (true, Some(SubMatch::new(matched_len_next, true)));
            }
        } else if pattern_s.starts_with(pinyin) {
            if pattern_s.len() == pinyin.len() {
                return (true, Some(SubMatch::new(matched_len_next, false)));
            }

            if let Some(submatch) = self.sub_test(
                &pattern[pinyin.chars().count()..],
                haystack_next,
                matched_len_next,
            ) {
                return (true, Some(submatch));
            }

            return (true, None);
        }

        (false, None)
    }

    // /// Reduce ~10% miss case time at the cost of some hit case time.
    // fn is_haystack_too_short_with_pattern(
    //     _pattern: &[PatternChar],
    //     _haystack: &HaystackStr,
    // ) -> bool {
    //     // For hit case:
    //     // - ~~A PatternChar must at least match one char in the haystack, i.e. `haystack.chars_count() >= pattern.len()`~~
    //     //  - ~~So `haystack.len() >= haystack.chars_count() >= pattern.len()`~~
    //     // - pattern.len() and pattern.s.len() may be shorter, equal, or longer than haystack.len()
    //     //   - We have pinyin that is longer than its hanzi, like "shuang".len() > "双".len()

    //     // haystack.chars_count() < pattern.len()
    //     // haystack.as_bytes().len() < pattern.len()
    //     false
    // }

    /// Already tested in match methods.
    pub fn is_haystack_too_short(&self, haystack: &HaystackStr) -> bool {
        // Self::is_haystack_too_short_with_pattern(&self.pattern, haystack)
        haystack.as_bytes().len() < self.min_haystack_chars
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
            PinyinMatcherBuilder::<str>::ORDERED_PINYIN_NOTATIONS.len()
        )
    }

    #[ignore]
    #[test]
    fn is_haystack_too_short() {
        // assert!(PinyinMatcher::is_haystack_too_short_with_pattern(&[], "") == false);
        // assert!(PinyinMatcher::is_haystack_too_short_with_pattern(&[], "a") == false);

        let matcher = PinyinMatcher::builder("pysseve")
            .pinyin_notations(PinyinNotation::Ascii)
            .build();
        assert!(matcher.is_haystack_too_short(""));
        assert!(matcher.is_haystack_too_short("a"));
        assert!(matcher.is_haystack_too_short("pyss"));
        assert!(matcher.is_haystack_too_short("pyssEverything") == false);
        assert!(matcher.is_haystack_too_short("拼"));
        assert!(matcher.is_haystack_too_short("拼音"));
        assert!(matcher.is_haystack_too_short("拼音搜") == false);
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

    #[cfg(feature = "encoding")]
    #[test]
    fn test_u16() {
        use widestring::u16str;

        let matcher = PinyinMatcher::builder(u16str!("xing"))
            .pinyin_notations(PinyinNotation::Ascii)
            .build();
        assert_match(matcher.test(u16str!("")), None);
        assert_match(matcher.test(u16str!("xing")), Some((0, 4)));
        assert_match(matcher.test(u16str!("XiNG")), Some((0, 4)));
        assert_match(matcher.test(u16str!("行")), Some((0, 1)));

        let matcher = PinyinMatcher::builder(u16str!("ke"))
            .pinyin_notations(PinyinNotation::Ascii)
            .build();
        assert_match(matcher.test(u16str!("ke")), Some((0, 2)));
        assert_match(matcher.test(u16str!("科")), Some((0, 1)));
        assert_match(matcher.test(u16str!("k鹅")), Some((0, 2)));
        assert_match(matcher.test(u16str!("凯尔")), None);

        let matcher = PinyinMatcher::builder(u16str!(""))
            .pinyin_notations(PinyinNotation::Ascii)
            .build();
        assert_match(matcher.test(u16str!("")), Some((0, 0)));
        assert_match(matcher.test(u16str!("abc")), Some((0, 0)));

        let matcher = PinyinMatcher::builder(u16str!("ke"))
            .pinyin_notations(PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter)
            .build();
        assert_match(matcher.test(u16str!("ke")), Some((0, 2)));
        assert_match(matcher.test(u16str!("科")), Some((0, 1)));
        assert_match(matcher.test(u16str!("k鹅")), Some((0, 2)));
        assert_match(matcher.test(u16str!("凯尔")), Some((0, 2)));
        // AsciiFirstLetter is preferred
        assert_match(matcher.test(u16str!("柯尔")), Some((0, 2)));
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
