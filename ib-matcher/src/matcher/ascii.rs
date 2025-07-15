use aho_corasick::{Anchored, Input, StartKind};
use bon::bon;

use crate::matcher::Match;

/// For ASCII-only haystack optimization.
pub enum AsciiMatcher<const CHAR_LEN: usize = 1> {
    /// ASCII-only haystack with non-ASCII pattern optimization
    Fail,
    /// - find_ascii_too_short: +170%
    ///   - TODO
    /// - is_match_ascii -50%
    /// - find_ascii -55%
    /// - build -60%, `build_analyze` -25%
    /// - Build size -837.5 KiB
    Ac(AcMatcher),
    #[cfg(feature = "regex")]
    #[allow(unused)]
    Regex(regex::bytes::Regex),
}

use AsciiMatcher::*;

pub struct AcMatcher {
    ac: aho_corasick::AhoCorasick,
    /// `ac` also has `start_kind`, but here has free space so anyway
    starts_with: bool,
    ends_with: bool,
}

impl AcMatcher {
    #[inline]
    pub fn input<'h>(&self, haystack: &'h [u8]) -> Input<'h> {
        Input::new(haystack).anchored(if self.starts_with {
            Anchored::Yes
        } else {
            Anchored::No
        })
    }
}

#[bon]
impl<const CHAR_LEN: usize> AsciiMatcher<CHAR_LEN> {
    #[builder]
    pub fn new(
        #[builder(start_fn)] pattern: &[u8],
        #[builder(default = false)] case_insensitive: bool,
        #[builder(default = false)] starts_with: bool,
        #[builder(default = false)] ends_with: bool,
        #[builder(default = false)] no_plain: bool,
    ) -> Self {
        match !no_plain && pattern.is_ascii() {
            true => {
                // regex::bytes::RegexBuilder::new(&regex_utils::escape_bytes(pattern))
                //     .unicode(false)
                //     .case_insensitive(case_insensitive)
                //     .build()
                //     .unwrap(),
                Ac(AcMatcher {
                    ac: aho_corasick::AhoCorasick::builder()
                        .ascii_case_insensitive(case_insensitive)
                        .start_kind(if starts_with {
                            StartKind::Anchored
                        } else {
                            StartKind::Unanchored
                        })
                        .build(&[pattern])
                        .unwrap(),
                    starts_with,
                    ends_with,
                })
            }
            false => Fail,
        }
    }

    pub fn find(&self, haystack: &[u8]) -> Option<Match> {
        match self {
            Fail => None,
            Ac(ac) => {
                if ac.ends_with {
                    let start = if ac.starts_with {
                        0
                    } else {
                        haystack.len().saturating_sub(ac.ac.max_pattern_len())
                    };
                    ac.ac
                        .find_iter(ac.input(&haystack[start..]))
                        .filter(|m| start + m.end() == haystack.len())
                        .map(|m| Match {
                            start: start + m.start() / CHAR_LEN,
                            end: start + m.end() / CHAR_LEN,
                            is_pattern_partial: false,
                        })
                        .next()
                } else {
                    ac.ac.find(ac.input(haystack)).map(|m| Match {
                        start: m.start() / CHAR_LEN,
                        end: m.end() / CHAR_LEN,
                        is_pattern_partial: false,
                    })
                }
            }
            #[cfg(feature = "regex")]
            Regex(regex) => regex.find(haystack).map(|m| Match {
                start: m.start() / CHAR_LEN,
                end: m.end() / CHAR_LEN,
                is_pattern_partial: false,
            }),
        }
    }

    pub fn is_match(&self, haystack: &[u8]) -> bool {
        match self {
            Fail => false,
            Ac(ac) => {
                if ac.ends_with {
                    self.find(haystack).is_some()
                } else {
                    ac.ac.is_match(ac.input(haystack))
                }
            }
            #[cfg(feature = "regex")]
            Regex(regex) => regex.is_match(haystack),
        }
    }

    pub fn test(&self, haystack: &[u8]) -> Option<Match> {
        match self {
            Fail => None,
            Ac(ac) => {
                // TODO: Always use anchored?
                let input = ac.input(haystack);
                if ac.ends_with {
                    ac.ac
                        .find(input)
                        .filter(|m| m.start() == 0 && m.end() == haystack.len())
                        .map(|m| Match {
                            start: 0,
                            end: m.end() / CHAR_LEN,
                            is_pattern_partial: false,
                        })
                } else {
                    ac.ac.find(input).filter(|m| m.start() == 0).map(|m| Match {
                        start: 0,
                        end: m.end() / CHAR_LEN,
                        is_pattern_partial: false,
                    })
                }
            }
            // TODO: Use regex-automata's anchored searches?
            #[cfg(feature = "regex")]
            Regex(regex) => regex
                .find(haystack.as_bytes())
                .filter(|m| m.start() == 0)
                .map(|m| Match {
                    start: 0,
                    end: m.end() / CHAR_LEN,
                    is_pattern_partial: false,
                }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_match;

    use super::*;

    #[test]
    fn ends_with() {
        let matcher = AsciiMatcher::<1>::builder(b"abc")
            .case_insensitive(true)
            .ends_with(true)
            .build();
        assert!(matcher.is_match(b"abc"));
        assert!(!matcher.is_match(b"ab"));
        assert_match!(matcher.find(b"abcd"), None);
        assert!(!matcher.is_match(b"abcd"));
        assert!(matcher.is_match(b"ABC"));
        assert_match!(matcher.find(b"xyzabc"), Some((3, 3)));
        assert!(matcher.is_match(b"xyzabc"));
        assert!(!matcher.is_match(b"xyzab"));

        let matcher = AsciiMatcher::<1>::builder(b"abc")
            .case_insensitive(true)
            .ends_with(false)
            .build();
        assert!(matcher.is_match(b"abc"));
        assert!(!matcher.is_match(b"ab"));
        assert!(matcher.is_match(b"abcd"));
        assert!(matcher.is_match(b"ABC"));
        assert!(matcher.is_match(b"xyzabc"));
        assert!(!matcher.is_match(b"xyzab"));
    }

    #[test]
    fn starts_with() {
        let matcher = AsciiMatcher::<1>::builder(b"abc")
            .case_insensitive(true)
            .starts_with(true)
            .build();
        assert!(matcher.is_match(b"abc"));
        assert!(!matcher.is_match(b"ab"));
        assert!(matcher.is_match(b"abcd"));
        assert!(matcher.is_match(b"ABC"));
        assert!(!matcher.is_match(b"xyzabc"));
        assert!(!matcher.is_match(b"xyzab"));

        let matcher = AsciiMatcher::<1>::builder(b"abc")
            .case_insensitive(true)
            .starts_with(false)
            .build();
        assert!(matcher.is_match(b"abc"));
        assert!(!matcher.is_match(b"ab"));
        assert!(matcher.is_match(b"abcd"));
        assert!(matcher.is_match(b"ABC"));
        assert!(matcher.is_match(b"xyzabc"));
        assert!(!matcher.is_match(b"xyzab"));
    }
}
