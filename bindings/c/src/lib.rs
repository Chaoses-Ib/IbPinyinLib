//! TODO: winres

#[diplomat::bridge]
mod ffi {
    use std::ffi::CStr;

    use ::ib_pinyin::{minimal, pinyin::PinyinNotation};

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
    }
}
