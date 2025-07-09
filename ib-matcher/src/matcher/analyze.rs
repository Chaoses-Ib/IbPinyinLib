use std::cmp::min;

use bon::Builder;

use crate::{matcher::PinyinMatchConfig, pinyin::PinyinNotation};

#[derive(Builder)]
pub struct PatternAnalyzeConfig {
    /// For better [`PatternAnalyzer::used_notations`] and [`PatternAnalyzer::min_haystack_len`].
    ///
    /// ~65us (+90%, 220~1100 matches)
    traversal: bool,
}

impl PatternAnalyzeConfig {
    pub fn standard() -> Self {
        Self { traversal: true }
    }
}

impl Default for PatternAnalyzeConfig {
    fn default() -> Self {
        Self { traversal: false }
    }
}

pub(crate) struct PatternAnalyzer<'a> {
    pattern: &'a str,

    pinyin: &'a PinyinMatchConfig<'a>,
    pinyin_used_notations: PinyinNotation,

    #[cfg(test)]
    min_haystack_chars: usize,
    min_haystack_len: usize,
}

impl<'a> PatternAnalyzer<'a> {
    pub fn new(pattern: &'a str, pinyin: &'a PinyinMatchConfig) -> Self {
        // debug_assert_eq!(pattern, pattern.to_mono_lowercase());
        // TODO: Case
        Self {
            pattern,
            pinyin,
            pinyin_used_notations: PinyinNotation::empty(),
            #[cfg(test)]
            min_haystack_chars: 0,
            min_haystack_len: 0,
        }
    }

    #[cfg(test)]
    fn analyze_default(&mut self) {
        self.analyze(PatternAnalyzeConfig::default());
    }

    #[cfg(test)]
    fn analyze_std(&mut self) {
        self.analyze(PatternAnalyzeConfig::standard());
    }

    pub fn analyze(&mut self, config: PatternAnalyzeConfig) {
        if config.traversal {
            self.pinyin_used_notations = PinyinNotation::empty();

            #[cfg(test)]
            {
                self.min_haystack_chars = usize::MAX;
            }
            self.min_haystack_len = usize::MAX;

            self.sub_analyze(self.pattern, 0, 0);
        } else {
            self.pinyin_used_notations = self.pinyin.notations;

            // Traversal can give a better lower bound
            let min_haystack_chars = {
                match self.pinyin_used_notations.max_len() {
                    Some(max_len) => {
                        // - Ascii: "shuang" / 6 = 1, "a" / 6 = 1
                        self.pattern.len().div_ceil(max_len)
                    }
                    None => {
                        // If case_insensitive, pattern length in bytes may be shorter than the matched haystack (or not?), so we use char count only
                        self.pattern.len()
                    }
                }
            };
            #[cfg(test)]
            {
                self.min_haystack_chars = min_haystack_chars;
            }
            self.min_haystack_len = min_haystack_chars;
        }
    }

    fn sub_analyze(&mut self, pattern: &str, depth: usize, min_len: usize) {
        if pattern.is_empty() {
            #[cfg(test)]
            if depth < self.min_haystack_chars {
                self.min_haystack_chars = depth;
            }
            if min_len < self.min_haystack_len {
                #[cfg(test)]
                println!("{}min_haystack_len: {min_len}", " ".repeat(depth));
                self.min_haystack_len = min_len;
            }
            return;
        }
        let c = pattern.chars().next().unwrap();

        let mut any_matched_single_char = false;
        for notation in self.pinyin.notations.iter() {
            for matched in self.pinyin.data.match_pinyin(notation, pattern) {
                let mut matched_single_char = false;
                if matched.len() == 1 {
                    matched_single_char = true;

                    if notation == PinyinNotation::Ascii
                        && self
                            .pinyin
                            .notations
                            .contains(PinyinNotation::AsciiFirstLetter)
                    {
                        // Only let AsciiFirstLetter analyze to prune the tree
                        continue;
                    }
                } else if self.pinyin.notations.contains(PinyinNotation::Unicode)
                    && matched.chars().nth(1).is_none()
                {
                    matched_single_char = true;
                }
                any_matched_single_char |= matched_single_char;

                self.pinyin_used_notations |= notation;

                // `MAX_RANGE` starts from 0x3007, at least 3 bytes
                let min_len = min_len
                    + if matched_single_char {
                        min(3, c.len_utf8())
                    } else {
                        3
                    };

                #[cfg(test)]
                println!(
                    "{}{matched} {:X} min_len={min_len} single={matched_single_char}",
                    " ".repeat(depth),
                    notation.bits()
                );

                self.sub_analyze(&pattern[matched.len()..], depth + 1, min_len);
            }
        }

        // Prune the tree
        if !any_matched_single_char {
            let matched = c;
            #[cfg(test)]
            println!("{}{matched}", " ".repeat(depth));
            let len = matched.len_utf8();
            self.sub_analyze(&pattern[len..], depth + 1, min_len + len);
        }
    }

    /// - If [`PinyinNotation::Ascii`] and [`PinyinNotation::AsciiFirstLetter`] are both enabled, [`PinyinNotation::Ascii`] is only considered used if the pattern uses any non-single-letter pinyin from [`PinyinNotation::Ascii`].
    pub fn pinyin_used_notations(&self) -> PinyinNotation {
        self.pinyin_used_notations
    }

    #[cfg(test)]
    pub fn min_haystack_chars(&self) -> usize {
        self.min_haystack_chars
    }

    pub fn min_haystack_len(&self) -> usize {
        self.min_haystack_len
    }

    #[cfg(test)]
    fn tree(&self) {
        self.sub_tree(self.pattern, 0)
    }

    #[cfg(test)]
    fn sub_tree(&self, pattern: &str, depth: usize) {
        if pattern.is_empty() {
            return;
        }
        for notation in self.pinyin.notations.iter() {
            for matched in self.pinyin.data.match_pinyin(notation, pattern) {
                println!("{}{matched} {:X}", " ".repeat(depth), notation.bits());
                self.sub_tree(&pattern[matched.len()..], depth + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pinyin::PinyinData;

    use super::*;

    #[ignore]
    #[test]
    fn tree() {
        let notations = PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter;
        let pinyin_data = PinyinData::new(notations);
        let pinyin = PinyinMatchConfig::builder(notations)
            .data(&pinyin_data)
            .build();
        let pattern = "pysousuoeve";
        let analyzer = PatternAnalyzer::new(pattern, &pinyin);
        analyzer.tree();
    }

    #[test]
    fn used_notations() {
        let pinyin_data = PinyinData::new(PinyinNotation::all());
        let pinyin =
            PinyinMatchConfig::builder(PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter)
                .data(&pinyin_data)
                .build();

        let mut analyzer = PatternAnalyzer::new("pysousuoeve", &pinyin);
        analyzer.analyze_std();
        assert_eq!(
            analyzer.pinyin_used_notations(),
            PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter
        );

        let mut analyzer = PatternAnalyzer::new("pyssEve", &pinyin);
        analyzer.analyze_std();
        assert_eq!(
            analyzer.pinyin_used_notations(),
            PinyinNotation::AsciiFirstLetter
        );
    }

    #[test]
    fn min_haystack_len() {
        let pinyin_data = PinyinData::new(PinyinNotation::all());
        let pinyin =
            PinyinMatchConfig::builder(PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter)
                .data(&pinyin_data)
                .build();

        let mut analyzer = PatternAnalyzer::new("pysousuoeve", &pinyin);
        analyzer.analyze_default();
        assert_eq!(analyzer.min_haystack_chars(), 2);
        assert_eq!(analyzer.min_haystack_len(), 2);
        analyzer.analyze_std();
        assert_eq!(analyzer.min_haystack_chars(), 7);
        assert_eq!(analyzer.min_haystack_len(), 11);

        let mut analyzer = PatternAnalyzer::new("pyssEve", &pinyin);
        analyzer.analyze_default();
        assert_eq!(analyzer.min_haystack_chars(), 2);
        assert_eq!(analyzer.min_haystack_len(), 2);
        analyzer.analyze_std();
        assert_eq!(analyzer.min_haystack_chars(), 7);
        assert_eq!(analyzer.min_haystack_len(), 7);

        let mut analyzer = PatternAnalyzer::new("pysseve", &pinyin);
        analyzer.analyze_default();
        assert_eq!(analyzer.min_haystack_chars(), 2);
        assert_eq!(analyzer.min_haystack_len(), 2);
        analyzer.analyze_std();
        assert_eq!(analyzer.min_haystack_chars(), 6);
        assert_eq!(analyzer.min_haystack_len(), 7);
    }
}
