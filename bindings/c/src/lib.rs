#[diplomat::bridge]
mod ffi {
    use std::ffi::CStr;

    use ::ib_pinyin::{minimal, pinyin::PinyinNotation};
    use widestring::{U16CStr, U16Str, U32CStr, U32Str};

    /// https://github.com/rust-diplomat/diplomat/issues/392
    #[allow(non_camel_case_types)]
    #[diplomat::opaque]
    pub struct ib_pinyin;

    impl ib_pinyin {
        pub fn is_match_u8(pattern: &str, haystack: &str, pinyin_notations: u32) -> bool {
            minimal::is_pinyin_match(
                pattern,
                haystack,
                PinyinNotation::from_bits_truncate(pinyin_notations),
            )
        }

        /// TODO: Lossy decoding?
        pub fn is_match_u8c(pattern: &u8, haystack: &u8, pinyin_notations: u32) -> bool {
            (|| -> Result<bool, std::str::Utf8Error> {
                Ok(Self::is_match_u8(
                    unsafe { CStr::from_ptr(pattern as *const _ as *const i8) }.to_str()?,
                    unsafe { CStr::from_ptr(haystack as *const _ as *const i8) }.to_str()?,
                    pinyin_notations,
                ))
            })()
            .unwrap_or(false)
        }

        pub fn is_match_u16(
            pattern: &u16,
            pattern_len: usize,
            haystack: &u16,
            haystack_len: usize,
            pinyin_notations: u32,
        ) -> bool {
            minimal::is_pinyin_match_u16(
                unsafe { U16Str::from_ptr(pattern as *const u16, pattern_len) },
                unsafe { U16Str::from_ptr(haystack as *const u16, haystack_len) },
                PinyinNotation::from_bits_truncate(pinyin_notations),
            )
        }

        pub fn is_match_u16c(pattern: &u16, haystack: &u16, pinyin_notations: u32) -> bool {
            minimal::is_pinyin_match_u16(
                unsafe { U16CStr::from_ptr_str(pattern as *const u16) }.as_ustr(),
                unsafe { U16CStr::from_ptr_str(haystack as *const u16) }.as_ustr(),
                PinyinNotation::from_bits_truncate(pinyin_notations),
            )
        }

        pub fn is_match_u32(
            pattern: &u32,
            pattern_len: usize,
            haystack: &u32,
            haystack_len: usize,
            pinyin_notations: u32,
        ) -> bool {
            minimal::is_pinyin_match_u32(
                unsafe { U32Str::from_ptr(pattern as *const u32, pattern_len) },
                unsafe { U32Str::from_ptr(haystack as *const u32, haystack_len) },
                PinyinNotation::from_bits_truncate(pinyin_notations),
            )
        }

        pub fn is_match_u32c(pattern: &u32, haystack: &u32, pinyin_notations: u32) -> bool {
            minimal::is_pinyin_match_u32(
                unsafe { U32CStr::from_ptr_str(pattern as *const u32) }.as_ustr(),
                unsafe { U32CStr::from_ptr_str(haystack as *const u32) }.as_ustr(),
                PinyinNotation::from_bits_truncate(pinyin_notations),
            )
        }
    }
}
