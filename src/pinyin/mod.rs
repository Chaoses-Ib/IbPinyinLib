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
/// - `Pinyin` does not require extra memory.
/// - `PinyinAsciiInitial` uses the same storage as `PinyinAscii`.
///
/// ## Others
/// TODO: Optionally generate pinyin notation data at build time.
///
/// TODO: Is row-major order more cache friendly?
///
/// Row-major order requires 7 * 8 * 1514 ≈ 83 KiB, dynamic columns are needed to reduce memory usage. And dynamic columns can also offer better cache locality.
///
/// TODO: Order pinyin by frequency to improve cache locality?
#[derive(Clone)]
pub struct PinyinData {
    inited_notations: PinyinNotation,
    pinyin_ascii: Option<Box<[PinyinString]>>,
    // TODO
    // pinyin_ascii_tone: Option<Box<[PinyinString]>>,
    // pinyin_ascii_initial: Option<Box<[u8]>>,
    // TODO
    // diletter_abc: Option<Box<[PinyinString]>>,
    // diletter_jiajia: Option<Box<[PinyinString]>>,
    // diletter_microsoft: Option<Box<[PinyinString]>>,
    // diletter_thunisoft: Option<Box<[PinyinString]>>,
    // diletter_xiaohe: Option<Box<[PinyinString]>>,
    // diletter_zrm: Option<Box<[PinyinString]>>,
}

impl PinyinData {
    pub fn new(notations: PinyinNotation) -> Self {
        let mut pinyin_data = Self {
            inited_notations: PinyinNotation::Pinyin,
            pinyin_ascii: None,
            // pinyin_ascii_tone: None,
            // pinyin_ascii_initial: None,
        };

        pinyin_data.init_notations(notations);
        pinyin_data
    }

    pub fn init_notations(&mut self, notations: PinyinNotation) {
        for notation in notations.iter() {
            match notation {
                PinyinNotation::Pinyin => (),
                PinyinNotation::PinyinAscii => {
                    self.pinyin_ascii.get_or_insert_with(|| {
                        data::PINYINS
                            .iter()
                            .map(|py| notation::pinyin_to_pinyin_ascii(py))
                            .collect::<Vec<_>>()
                            .into_boxed_slice()
                    });
                }
                PinyinNotation::PinyinAsciiTone => todo!(),
                PinyinNotation::PinyinAsciiInitial => {
                    self.init_notations(PinyinNotation::PinyinAscii)
                }
                _ => todo!(),
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
            PinyinNotation::Pinyin => Some(data::PINYINS[i]),
            PinyinNotation::PinyinAscii => self.data.pinyin_ascii.as_ref().map(|py| py[i].as_str()),
            // PinyinNotation::PinyinAsciiTone => {}
            PinyinNotation::PinyinAsciiInitial => self
                .data
                .pinyin_ascii
                .as_ref()
                .map(|py| unsafe { py[i].as_str().get_unchecked(..1) }),
            // PinyinNotation::DiletterAbc => {}
            // PinyinNotation::DiletterJiajia => {}
            // PinyinNotation::DiletterMicrosoft => {}
            // PinyinNotation::DiletterThunisoft => {}
            // PinyinNotation::DiletterXiaohe => {}
            // PinyinNotation::DiletterZrm => {}
            _ => None,
        }
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
        let data = PinyinData::new(PinyinNotation::PinyinAscii);

        assert_eq!(data.get_pinyins('中').count(), 2);

        for pinyin in data.get_pinyins('中') {
            println!("{:?}", pinyin);

            for notation in [
                PinyinNotation::Pinyin,
                PinyinNotation::PinyinAscii,
                PinyinNotation::PinyinAsciiInitial,
            ] {
                assert!(pinyin.notation(notation).is_some_and(|py| !py.is_empty()));
            }
        }
    }
}