//! Minimal APIs

use std::sync::{OnceLock, RwLock};

use crate::{
    matcher::PinyinMatcher,
    pinyin::{PinyinData, PinyinNotation},
};

fn pinyin_data() -> &'static PinyinData {
    static PINYIN_DATA: OnceLock<PinyinData> = OnceLock::new();
    PINYIN_DATA.get_or_init(|| PinyinData::new(PinyinNotation::empty()))
}

pub fn is_pinyin_match(pattern: &str, haystack: &str, pinyin_notations: PinyinNotation) -> bool {
    struct MatcherCache {
        pattern: String,
        pinyin_notations: PinyinNotation,
        matcher: PinyinMatcher<'static>,
    }

    static MATCHER_CACHE: OnceLock<RwLock<MatcherCache>> = OnceLock::new();
    let init = || MatcherCache {
        pattern: pattern.to_owned(),
        pinyin_notations,
        matcher: PinyinMatcher::builder(pattern)
            .pinyin_data(pinyin_data())
            .pinyin_notations(pinyin_notations)
            .build(),
    };
    let lock = MATCHER_CACHE.get_or_init(|| RwLock::new(init()));
    let cache = {
        let guard = lock.read().unwrap();
        if guard.pattern == pattern && guard.pinyin_notations == pinyin_notations {
            guard
        } else {
            drop(guard);
            *lock.write().unwrap() = init();
            lock.read().unwrap()
        }
    };

    cache.matcher.is_match(haystack)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_pinyin_match_() {
        // 0x3
        let notation = PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter;
        assert!(is_pinyin_match("xing", "", notation) == false);
        assert!(is_pinyin_match("xing", "xing", notation));
        assert!(is_pinyin_match("xing", "XiNG", notation));
        assert!(is_pinyin_match("xing", "行", notation));
        assert!(is_pinyin_match("XING", "行", notation) == false);

        assert!(is_pinyin_match("", "", notation));
        assert!(is_pinyin_match("", "abc", notation));
    }
}
