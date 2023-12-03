use super::PinyinString;

bitflags::bitflags! {
    /// ## Others
    /// TODO: doc alias does not work
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct PinyinNotation: u32 {
        /// e.g. "pīn", "yīn"
        const Unicode = 0x1;

        /// 全拼
        ///
        /// e.g. "pin", "yin"
        ///
        /// See [全拼](https://zh.wikipedia.org/wiki/全拼) for details.
        #[doc(alias = "全拼")]
        const Ascii = 0x2;

        /// 带声调全拼
        ///
        /// The tone digit is in `1..=5`. See [tones](https://en.wikipedia.org/wiki/Pinyin#Tones) for details.
        ///
        /// e.g. "pin1", "yin1"
        #[doc(alias = "带声调全拼")]
        const AsciiTone = 0x4;

        /// 简拼
        ///
        /// e.g. "p", "y"
        ///
        /// See [简拼](https://zh.wikipedia.org/wiki/简拼) for details.
        #[doc(alias = "简拼")]
        const AsciiFirstLetter = 0x8;

        /// 智能 ABC 双拼
        ///
        /// See [智能ABC输入法](https://zh.wikipedia.org/wiki/智能ABC输入法#双拼方案) for details.
        #[doc(alias = "智能ABC双拼")]
        const DiletterAbc = 0x10;

        /// 拼音加加双拼
        ///
        /// See [拼音加加](https://zh.wikipedia.org/wiki/拼音加加#双拼方案) for details.
        #[doc(alias = "拼音加加双拼")]
        const DiletterJiajia = 0x20;

        /// 微软双拼
        ///
        /// See [微软拼音输入法](https://zh.wikipedia.org/wiki/微软拼音输入法#双拼方案) for details.
        #[doc(alias = "微软双拼")]
        const DiletterMicrosoft = 0x40;

        /// 华宇双拼（紫光双拼）
        ///
        /// See [华宇拼音输入法](https://zh.wikipedia.org/wiki/华宇拼音输入法#双拼方案) for details.
        #[doc(alias("华宇双拼", "紫光双拼"))]
        const DiletterThunisoft = 0x80;

        /// 小鹤双拼
        ///
        /// See [小鹤双拼](https://flypy.com/) for details.
        #[doc(alias = "小鹤双拼")]
        const DiletterXiaohe = 0x100;

        /// 自然码双拼
        ///
        /// See [自然码](https://zh.wikipedia.org/zh-cn/自然码) for details.
        #[doc(alias = "自然码双拼")]
        const DiletterZrm = 0x200;
    }
}

pub(super) fn unicode_to_ascii(unicode: &str) -> PinyinString {
    let mut ascii = PinyinString::new();
    let mut chars = unicode.chars();
    while let Some(c) = chars.next() {
        match c {
            'a'..='z' => {
                ascii.extend([c]);
                let mut chars_try_next = chars.clone();
                if let Some(c) = chars_try_next.next() {
                    if c == '̀' {
                        chars = chars_try_next;
                    }
                }
            }
            _ => ascii.extend([match c {
                'ā' | 'á' | 'ǎ' | 'à' => b'a',
                'ē' | 'é' | 'ě' | 'è' | 'ế' | 'ề' => b'e',
                // "ê̄" | "ê̌" => b'e',
                'ê' => {
                    let c = chars.next();
                    assert!(c == Some('̄') || c == Some('̌'));
                    b'e'
                }
                'ī' | 'í' | 'ǐ' | 'ì' => b'i',
                'ō' | 'ó' | 'ǒ' | 'ò' => b'o',
                'ū' | 'ú' | 'ǔ' | 'ù' => b'u',
                'ü' | 'ǘ' | 'ǚ' | 'ǜ' => b'v',
                'ń' | 'ň' | 'ǹ' => b'n',
                'ḿ' => b'm',
                // "m̀" begins with 'm'
                // "m̀" => b'm',
                _ => unreachable!(),
            } as char]),
        }
    }
    ascii
}

#[cfg(test)]
mod tests {
    use super::{super::data, *};

    #[test]
    #[ignore]
    fn unicode_to_ascii_() {
        for unicode in data::PINYINS {
            println!("{}: {}", unicode, unicode_to_ascii(unicode));
        }
    }
}
