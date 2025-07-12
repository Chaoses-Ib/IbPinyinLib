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
    Ac(aho_corasick::AhoCorasick),
    #[cfg(feature = "regex")]
    #[allow(unused)]
    Regex(regex::bytes::Regex),
}

use AsciiMatcher::*;

#[bon]
impl<const CHAR_LEN: usize> AsciiMatcher<CHAR_LEN> {
    #[builder]
    pub fn new(
        #[builder(start_fn)] pattern: &[u8],
        #[builder(default = false)] case_insensitive: bool,
    ) -> Self {
        match pattern.is_ascii() {
            true => {
                // regex::bytes::RegexBuilder::new(&regex_utils::escape_bytes(pattern))
                //     .unicode(false)
                //     .case_insensitive(case_insensitive)
                //     .build()
                //     .unwrap(),
                Ac(aho_corasick::AhoCorasick::builder()
                    .ascii_case_insensitive(case_insensitive)
                    .build(&[pattern])
                    .unwrap())
            }
            false => Fail,
        }
    }

    pub fn find(&self, haystack: &[u8]) -> Option<Match> {
        match self {
            Fail => None,
            Ac(ac) => ac.find(haystack).map(|m| Match {
                start: m.start() / CHAR_LEN,
                end: m.end() / CHAR_LEN,
                is_pattern_partial: false,
            }),
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
            Ac(ac) => ac.is_match(haystack),
            #[cfg(feature = "regex")]
            Regex(regex) => regex.is_match(haystack),
        }
    }

    pub fn test(&self, haystack: &[u8]) -> Option<Match> {
        match self {
            Fail => None,
            Ac(ac) => ac.find(haystack).filter(|m| m.start() == 0).map(|m| Match {
                start: 0,
                end: m.end() / CHAR_LEN,
                is_pattern_partial: false,
            }),
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
