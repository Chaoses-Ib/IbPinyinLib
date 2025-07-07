use bon::Builder;

use crate::pinyin::{PinyinData, PinyinNotation};

#[derive(Builder)]
pub struct PatternAnalyzeConfig {
    /// For [`PatternAnalyzer::used_notations`]
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
    pinyin_data: &'a PinyinData,
    notations: PinyinNotation,

    used_notations: PinyinNotation,
}

impl<'a> PatternAnalyzer<'a> {
    pub fn new(pattern: &'a str, pinyin_data: &'a PinyinData, notations: PinyinNotation) -> Self {
        // debug_assert_eq!(pattern, pattern.to_mono_lowercase());
        // TODO: Case
        Self {
            pattern,
            pinyin_data,
            notations,
            used_notations: PinyinNotation::empty(),
        }
    }

    #[cfg(test)]
    fn analyze_std(&mut self) {
        self.analyze(PatternAnalyzeConfig::standard());
    }

    pub fn analyze(&mut self, config: PatternAnalyzeConfig) {
        if config.traversal {
            self.sub_analyze(self.pattern, 0);
        } else {
            self.used_notations = self.notations;
        }
    }

    fn sub_analyze(&mut self, pattern: &str, depth: usize) {
        if pattern.is_empty() {
            return;
        }

        let mut matched_single_char = false;
        for notation in self.notations.iter() {
            for matched in self.pinyin_data.match_pinyin(notation, pattern) {
                if matched.len() == 1 {
                    matched_single_char = true;

                    if notation == PinyinNotation::Ascii
                        && self.notations.contains(PinyinNotation::AsciiFirstLetter)
                    {
                        // Only let AsciiFirstLetter analyze to prune the tree
                        continue;
                    }
                } else if self.notations.contains(PinyinNotation::Unicode)
                    && matched.chars().nth(1).is_none()
                {
                    matched_single_char = true;
                }
                self.used_notations |= notation;

                #[cfg(test)]
                println!("{}{matched} {:X}", " ".repeat(depth), notation.bits());
                self.sub_analyze(&pattern[matched.len()..], depth + 1);
            }
        }

        // Prune the tree
        if !matched_single_char {
            let matched = pattern.chars().next().unwrap();
            #[cfg(test)]
            println!("{}{matched}", " ".repeat(depth));
            self.sub_analyze(&pattern[matched.len_utf8()..], depth + 1);
        }
    }

    /// - If [`PinyinNotation::Ascii`] and [`PinyinNotation::AsciiFirstLetter`] are both enabled, [`PinyinNotation::Ascii`] is only considered used if the pattern uses any non-single-letter pinyin from [`PinyinNotation::Ascii`].
    pub fn used_notations(&self) -> PinyinNotation {
        self.used_notations
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
        for notation in self.notations.iter() {
            for matched in self.pinyin_data.match_pinyin(notation, pattern) {
                println!("{}{matched} {:X}", " ".repeat(depth), notation.bits());
                self.sub_tree(&pattern[matched.len()..], depth + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn tree() {
        let notations = PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter;
        let pinyin_data = PinyinData::new(notations);
        let pattern = "pysousuoeve";
        let analyzer = PatternAnalyzer::new(pattern, &pinyin_data, notations);
        analyzer.tree();
    }

    #[test]
    fn used_notations() {
        let pinyin_data = PinyinData::new(PinyinNotation::all());

        let mut analyzer = PatternAnalyzer::new(
            "pysousuoeve",
            &pinyin_data,
            PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter,
        );
        analyzer.analyze_std();
        assert_eq!(
            analyzer.used_notations(),
            PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter
        );

        let mut analyzer = PatternAnalyzer::new(
            "pyssEve",
            &pinyin_data,
            PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter,
        );
        analyzer.analyze_std();
        assert_eq!(analyzer.used_notations(), PinyinNotation::AsciiFirstLetter);
    }
}
