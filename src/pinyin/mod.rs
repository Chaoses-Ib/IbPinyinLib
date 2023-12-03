//! [Pinyin](https://en.wikipedia.org/wiki/Pinyin)

use std::ops::RangeInclusive;

mod data;
mod notation;

pub(super) type PinyinCombination = [u16; data::PINYIN_COMBINATION_LEN];

pub(super) struct PinyinRangeTable {
    range: RangeInclusive<u32>,
    /// Array of indices into `data::PINYINS` or `data::PINYIN_COMBINATIONS`.
    table: &'static [u16],
}

impl PinyinRangeTable {
    const MAX_RANGE: RangeInclusive<u32> = 0x3007..=0x30EDE;

    pub(super) const fn new(range: RangeInclusive<u32>, table: &'static [u16]) -> Self {
        PinyinRangeTable { range, table }
    }
}

pub use notation::PinyinNotation;

type PinyinString = arraystring::ArrayString<arraystring::typenum::U7>;

/// ## Memory usage
/// Per pinyin notation: 8 * 1514 ≈ 11.8 KiB.
/// - `Unicode` does not require extra memory.
/// - `AsciiFirstLetter` uses the same storage as `Ascii`.
///
/// ## Others
/// TODO: Optionally generate pinyin notation data at build time.
///
/// Less flexible at runtime.
///
/// TODO: Is row-major order more cache friendly?
///
/// Row-major order requires 7 * 8 * 1514 ≈ 83 KiB, dynamic columns are needed to reduce memory usage. And dynamic columns can also offer better cache locality.
///
/// TODO: Order pinyin by frequency to improve cache locality?
#[derive(Clone)]
pub struct PinyinData {
    inited_notations: PinyinNotation,
    ascii: Option<Box<[PinyinString]>>,
    // TODO
    // ascii_tone: Option<Box<[PinyinString]>>,
    // ascii_first_letter: Option<Box<[u8]>>,
    diletter_abc: Option<Box<[PinyinString]>>,
    diletter_jiajia: Option<Box<[PinyinString]>>,
    diletter_microsoft: Option<Box<[PinyinString]>>,
    diletter_thunisoft: Option<Box<[PinyinString]>>,
    diletter_xiaohe: Option<Box<[PinyinString]>>,
    diletter_zrm: Option<Box<[PinyinString]>>,
}

impl PinyinData {
    pub fn new(notations: PinyinNotation) -> Self {
        let mut pinyin_data = Self {
            inited_notations: PinyinNotation::Unicode,
            ascii: None,
            // ascii_tone: None,
            // ascii_first_letter: None,
            diletter_abc: None,
            diletter_jiajia: None,
            diletter_microsoft: None,
            diletter_thunisoft: None,
            diletter_xiaohe: None,
            diletter_zrm: None,
        };

        pinyin_data.init_notations(notations);
        pinyin_data
    }

    fn init_notation_with_ascii(
        ascii: &[PinyinString],
        map: impl Fn(&str) -> PinyinString,
        notation: &mut Option<Box<[PinyinString]>>,
    ) {
        notation.get_or_insert_with(|| {
            ascii
                .iter()
                .map(|py| map(py))
                .collect::<Vec<_>>()
                .into_boxed_slice()
        });
    }

    pub fn init_notations(&mut self, notations: PinyinNotation) {
        for notation in notations.iter() {
            match notation {
                PinyinNotation::Unicode => (),
                PinyinNotation::Ascii => {
                    self.ascii.get_or_insert_with(|| {
                        data::PINYINS
                            .iter()
                            .map(|py| notation::unicode_to_ascii(py))
                            .collect::<Vec<_>>()
                            .into_boxed_slice()
                    });
                }
                PinyinNotation::AsciiTone => todo!(),
                PinyinNotation::AsciiFirstLetter => self.init_notations(PinyinNotation::Ascii),

                PinyinNotation::DiletterAbc => Self::init_notation_with_ascii(
                    self.ascii.as_ref().unwrap(),
                    notation::ascii_to_diletter_abc,
                    &mut self.diletter_abc,
                ),
                PinyinNotation::DiletterJiajia => Self::init_notation_with_ascii(
                    self.ascii.as_ref().unwrap(),
                    notation::ascii_to_diletter_jiajia,
                    &mut self.diletter_jiajia,
                ),
                PinyinNotation::DiletterMicrosoft => Self::init_notation_with_ascii(
                    self.ascii.as_ref().unwrap(),
                    notation::ascii_to_diletter_microsoft,
                    &mut self.diletter_microsoft,
                ),
                PinyinNotation::DiletterThunisoft => Self::init_notation_with_ascii(
                    self.ascii.as_ref().unwrap(),
                    notation::ascii_to_diletter_thunisoft,
                    &mut self.diletter_thunisoft,
                ),
                PinyinNotation::DiletterXiaohe => Self::init_notation_with_ascii(
                    self.ascii.as_ref().unwrap(),
                    notation::ascii_to_diletter_xiaohe,
                    &mut self.diletter_xiaohe,
                ),
                PinyinNotation::DiletterZrm => Self::init_notation_with_ascii(
                    self.ascii.as_ref().unwrap(),
                    notation::ascii_to_diletter_zrm,
                    &mut self.diletter_zrm,
                ),
                _ => unreachable!(),
            }
        }
        self.inited_notations |= notations;
    }

    pub fn inited_notations(&self) -> PinyinNotation {
        self.inited_notations
    }

    fn get_pinyin_index(c: char) -> Option<u16> {
        if PinyinRangeTable::MAX_RANGE.contains(&(c as u32)) {
            for range in &data::PINYIN_RANGE_TABLES {
                if range.range.contains(&(c as u32)) {
                    return match range.table[(c as u32 - range.range.start()) as usize] {
                        u16::MAX => None,
                        i => Some(i),
                    };
                }
            }
        }
        None
    }

    fn pinyin_combination(index: u16) -> impl Iterator<Item = &'static u16> {
        data::PINYIN_COMBINATIONS[index as usize]
            .iter()
            .take_while(|&&i| i != u16::MAX)
    }

    fn pinyin(&self, index: u16) -> Pinyin {
        Pinyin { data: self, index }
    }

    /// Prefer `get_pinyins_and_for_each` if applicable.
    pub fn get_pinyins<'a>(&'a self, c: char) -> Box<dyn Iterator<Item = Pinyin<'a>> + 'a> {
        if let Some(i) = Self::get_pinyin_index(c) {
            if i < data::PINYINS.len() as u16 {
                Box::new([self.pinyin(i)].into_iter())
            } else {
                let i = i - data::PINYINS.len() as u16;
                Box::new(Self::pinyin_combination(i).map(|&i| self.pinyin(i)))
            }
        } else {
            Box::new([].into_iter())
        }
    }

    pub fn get_pinyins_and_for_each(&self, c: char, mut f: impl FnMut(Pinyin)) {
        if let Some(i) = Self::get_pinyin_index(c) {
            if i < data::PINYINS.len() as u16 {
                f(self.pinyin(i));
            } else {
                let i = i - data::PINYINS.len() as u16;
                Self::pinyin_combination(i).for_each(|&i| f(self.pinyin(i)));
            }
        }
    }
}

pub struct Pinyin<'a> {
    data: &'a PinyinData,
    index: u16,
}

impl<'a> Pinyin<'a> {
    pub fn notation(&self, notation: PinyinNotation) -> Option<&str> {
        debug_assert_eq!(notation.bits().count_ones(), 1);

        let i = self.index as usize;
        match notation {
            PinyinNotation::Unicode => Some(data::PINYINS[i]),
            PinyinNotation::Ascii => self.data.ascii.as_ref().map(|py| py[i].as_str()),
            // PinyinNotation::AsciiTone => {}
            PinyinNotation::AsciiFirstLetter => self
                .data
                .ascii
                .as_ref()
                .map(|py| unsafe { py[i].as_str().get_unchecked(..1) }),

            PinyinNotation::DiletterAbc => self.data.diletter_abc.as_ref().map(|py| py[i].as_str()),
            PinyinNotation::DiletterJiajia => {
                self.data.diletter_jiajia.as_ref().map(|py| py[i].as_str())
            }
            PinyinNotation::DiletterMicrosoft => self
                .data
                .diletter_microsoft
                .as_ref()
                .map(|py| py[i].as_str()),
            PinyinNotation::DiletterThunisoft => self
                .data
                .diletter_thunisoft
                .as_ref()
                .map(|py| py[i].as_str()),
            PinyinNotation::DiletterXiaohe => {
                self.data.diletter_xiaohe.as_ref().map(|py| py[i].as_str())
            }
            PinyinNotation::DiletterZrm => self.data.diletter_zrm.as_ref().map(|py| py[i].as_str()),
            _ => None,
        }
    }

    /// Require `PinyinNotation::Ascii`.
    pub fn initial_final(&self) -> Option<(&str, &str)> {
        self.notation(PinyinNotation::Ascii)
            .map(Self::split_initial_final)
    }

    fn split_initial_final(ascii: &str) -> (&str, &str) {
        debug_assert!(ascii.is_ascii());

        // TODO: as_ascii_unchecked
        let mut chars = ascii.chars();
        ascii.split_at(match chars.next().unwrap() {
            'a' | 'e' | 'i' | 'o' | 'u' | 'v' => 0,
            'z' | 'c' | 's' => {
                if chars.next().unwrap() == 'h' {
                    2
                } else {
                    1
                }
            }
            _ => 1,
        })
    }
}

impl core::fmt::Debug for Pinyin<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut s = f.debug_struct("Pinyin");
        for (name, notation) in PinyinNotation::all().iter_names() {
            if let Some(py) = self.notation(notation) {
                s.field(name, &py);
            }
        }
        s.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pinyin_max_len() {
        let max = data::PINYINS.iter().map(|p| p.len()).max().unwrap();
        assert_eq!(max, 7);
    }

    #[test]
    fn pinyin_range_tables() {
        // `Option::<T>::unwrap` is not yet stable as a const fn
        // const MIN_START: u32 = data::PINYIN_RANGE_TABLES
        //     .iter()
        //     .map(|range| *range.range.start())
        //     .min()
        //     .unwrap();
        // const MAX_END: u32 = data::PINYIN_RANGE_TABLES
        //     .iter()
        //     .map(|range| *range.range.end())
        //     .max()
        //     .unwrap();

        let min_start = data::PINYIN_RANGE_TABLES
            .iter()
            .map(|range| *range.range.start())
            .min()
            .unwrap();
        let max_end = data::PINYIN_RANGE_TABLES
            .iter()
            .map(|range| *range.range.end())
            .max()
            .unwrap();
        assert_eq!(min_start..=max_end, PinyinRangeTable::MAX_RANGE)
    }

    #[test]
    fn get_pinyins() {
        let data = PinyinData::new(PinyinNotation::all() - PinyinNotation::AsciiTone);

        assert_eq!(data.get_pinyins('中').count(), 2);

        for pinyin in data.get_pinyins('中') {
            println!("{:?}", pinyin);

            for notation in (PinyinNotation::all() - PinyinNotation::AsciiTone).iter() {
                assert!(pinyin.notation(notation).is_some_and(|py| !py.is_empty()));
            }
        }
    }
}
