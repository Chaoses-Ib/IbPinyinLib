use super::PinyinString;

bitflags::bitflags! {
    /// ## Others
    /// TODO: doc alias does not work
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct PinyinNotation: u32 {
        /// e.g. "pīn", "yīn"
        const Pinyin = 0x1;

        /// 全拼
        ///
        /// e.g. "pin", "yin"
        #[doc(alias = "全拼")]
        const PinyinAscii = 0x2;

        /// 带声调全拼
        ///
        /// The tone digit is in `1..=5`. See [tones](https://en.wikipedia.org/wiki/Pinyin#Tones) for details.
        ///
        /// e.g. "pin1", "yin1"
        #[doc(alias = "带声调全拼")]
        const PinyinAsciiTone = 0x4;

        /// 简拼
        ///
        /// e.g. "p", "y"
        #[doc(alias = "简拼")]
        const PinyinAsciiInitial = 0x8;

        /// 智能 ABC 双拼
        #[doc(alias = "智能ABC双拼")]
        const DiletterAbc = 0x10;

        /// 拼音加加双拼
        #[doc(alias = "拼音加加双拼")]
        const DiletterJiajia = 0x20;

        /// 微软双拼
        #[doc(alias = "微软双拼")]
        const DiletterMicrosoft = 0x40;

        /// 华宇双拼（紫光双拼）
        #[doc(alias("华宇双拼", "紫光双拼"))]
        const DiletterThunisoft = 0x80;

        /// 小鹤双拼
        #[doc(alias = "小鹤双拼")]
        const DiletterXiaohe = 0x100;

        /// 自然码双拼
        #[doc(alias = "自然码双拼")]
        const DiletterZrm = 0x200;
    }
}

pub(super) fn pinyin_to_pinyin_ascii(pinyin: &str) -> PinyinString {
    let mut pinyin_ascii = PinyinString::new();
    let mut chars = pinyin.chars();
    while let Some(c) = chars.next() {
        match c {
            'a'..='z' => {
                pinyin_ascii.extend([c]);
                let mut chars_try_next = chars.clone();
                if let Some(c) = chars_try_next.next() {
                    if c == '̀' {
                        chars = chars_try_next;
                    }
                }
            }
            _ => pinyin_ascii.extend([match c {
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
    pinyin_ascii
}

#[cfg(test)]
mod tests {
    use super::{super::data, *};

    #[test]
    #[ignore]
    fn pinyin_to_pinyin_ascii_() {
        for pinyin in data::PINYINS {
            println!("{}: {}", pinyin, pinyin_to_pinyin_ascii(pinyin));
        }
    }
}
