//! Minimal APIs
//!
//! TODO: Thread-local cache

use std::sync::{OnceLock, RwLock, RwLockReadGuard};

use crate::{
    matcher::{encoding::EncodedStr, PinyinMatcher},
    pinyin::{PinyinData, PinyinNotation},
};

pub fn pinyin_data() -> &'static PinyinData {
    static PINYIN_DATA: OnceLock<PinyinData> = OnceLock::new();
    PINYIN_DATA.get_or_init(|| PinyinData::new(PinyinNotation::empty()))
}

// Type maps have a cost
// fn with_cached_matcher<'a, HaystackStr, R>(
//     pattern: &'a HaystackStr,
//     pinyin_notations: PinyinNotation,
//     f: impl FnOnce(&PinyinMatcher<'static, HaystackStr>) -> R,
// ) -> R
// where
//     HaystackStr: EncodedStr + ?Sized + Clone + 'static,
//     <HaystackStr as ToOwned>::Owned: PartialEq<&'a HaystackStr>,
//     R: Default,
// {
//     struct MatcherCache<HaystackStr>
//     where
//         HaystackStr: EncodedStr + ?Sized + Clone,
//     {
//         pattern: <HaystackStr as ToOwned>::Owned,
//         pinyin_notations: PinyinNotation,
//         matcher: PinyinMatcher<'static, HaystackStr>,
//     }

//     let init = || MatcherCache {
//         pattern: pattern.to_owned(),
//         pinyin_notations,
//         matcher: PinyinMatcher::builder(pattern)
//             .pinyin_data(pinyin_data())
//             .pinyin_notations(pinyin_notations)
//             .build(),
//     };
//     // TODO: with() should return R
//     let mut r = Default::default();
//     generic_singleton::get_or_init_thread_local!(|| Cell::new(init()), |cell| {
//         let mut matcher = unsafe { &*cell.as_ptr() };
//         if matcher.pattern != pattern || matcher.pinyin_notations != pinyin_notations {
//             cell.set(init());
//             matcher = unsafe { &*cell.as_ptr() };
//         }
//         r = f(&matcher.matcher);
//     });
//     r
// }

struct MatcherCache<HaystackStr>
where
    HaystackStr: EncodedStr + ?Sized + ToOwned,
{
    pattern: <HaystackStr as ToOwned>::Owned,
    pinyin_notations: PinyinNotation,
    matcher: PinyinMatcher<'static, HaystackStr>,
}

fn get_or_init_matcher_cache<'a, HaystackStr>(
    matcher_cache: &'static OnceLock<RwLock<MatcherCache<HaystackStr>>>,
    pattern: &'a HaystackStr,
    pinyin_notations: PinyinNotation,
) -> RwLockReadGuard<'static, MatcherCache<HaystackStr>>
where
    HaystackStr: EncodedStr + ?Sized + ToOwned + 'static,
    <HaystackStr as ToOwned>::Owned: PartialEq<&'a HaystackStr>,
{
    let init = || MatcherCache {
        pattern: pattern.to_owned(),
        pinyin_notations,
        matcher: PinyinMatcher::builder(pattern)
            .pinyin_data(pinyin_data())
            .pinyin_notations(pinyin_notations)
            .build(),
    };

    let lock = matcher_cache.get_or_init(|| RwLock::new(init()));

    let guard = lock.read().unwrap();
    if guard.pattern == pattern && guard.pinyin_notations == pinyin_notations {
        guard
    } else {
        drop(guard);
        *lock.write().unwrap() = init();
        lock.read().unwrap()
    }
}

pub fn is_pinyin_match(pattern: &str, haystack: &str, pinyin_notations: PinyinNotation) -> bool {
    static MATCHER_CACHE: OnceLock<RwLock<MatcherCache<str>>> = OnceLock::new();
    get_or_init_matcher_cache(&MATCHER_CACHE, pattern, pinyin_notations)
        .matcher
        .is_match(haystack)
}

#[cfg(feature = "encoding")]
pub fn is_pinyin_match_u16(
    pattern: &widestring::U16Str,
    haystack: &widestring::U16Str,
    pinyin_notations: PinyinNotation,
) -> bool {
    static MATCHER_CACHE: OnceLock<RwLock<MatcherCache<widestring::U16Str>>> = OnceLock::new();
    get_or_init_matcher_cache(&MATCHER_CACHE, pattern, pinyin_notations)
        .matcher
        .is_match(haystack)
}

#[cfg(feature = "encoding")]
pub fn is_pinyin_match_u32(
    pattern: &widestring::U32Str,
    haystack: &widestring::U32Str,
    pinyin_notations: PinyinNotation,
) -> bool {
    static MATCHER_CACHE: OnceLock<RwLock<MatcherCache<widestring::U32Str>>> = OnceLock::new();
    get_or_init_matcher_cache(&MATCHER_CACHE, pattern, pinyin_notations)
        .matcher
        .is_match(haystack)
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

    #[cfg(feature = "encoding")]
    #[test]
    fn is_pinyin_match_u16_() {
        is_pinyin_match_u16(
            widestring::u16str!("zuo"),
            widestring::u16str!("协作"),
            PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter,
        );
    }
}
