//! A fast Japanese romanizer.
//!
//! The dictionary will take ~5.5 MiB in the binary at the moment.
//!
//! ## Design
//! `&[&str]` will cause each str to occupy 16 extra bytes to store the pointer and length. While CStr only needs 1 byte for each str.
//! - For words, this can save 3.14 MiB (actually 3.54 MiB).
//!   - Source file: 2.98 MiB -> `\0`+`\`: 2.80 MiB, `\n`: 2.54 MiB
//!   - `build()` time: `split()`/memchr +10%
//! - And this way the str can also be compressed and then streamly decompressed.
//!
//! ## Features
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(feature = "doc", doc = document_features::document_features!())]
use bon::bon;
use daachorse::{CharwiseDoubleArrayAhoCorasick, CharwiseDoubleArrayAhoCorasickBuilder, MatchKind};

use ib_unicode::str::floor_char_boundary;

mod data;

/// [Hepburn romanization](https://en.wikipedia.org/wiki/Hepburn_romanization)
#[derive(Clone)]
pub struct HepburnRomanizer {
    // ac: AhoCorasick,
    ac: CharwiseDoubleArrayAhoCorasick<u32>,
    kanji: bool,
}

#[bon]
impl HepburnRomanizer {
    /// [`HepburnRomanizer::default()`]
    #[builder]
    pub fn new(
        #[builder(default = false)] kana: bool,
        #[builder(default = false)] kanji: bool,
        #[builder(default = false)] word: bool,
    ) -> Self {
        // // let start = UnsafeCell::new(0);
        // let mut start = 0;
        // let words = memchr::memchr_iter(b'\n', data::WORDS.as_bytes()).map(|end| {
        //     // let start = start.get();
        //     // let word = unsafe { str::from_raw_parts(data::WORDS.as_ptr().add(start), end - start) };
        //     let word = unsafe { data::WORDS.get_unchecked(start..end) };
        //     start = end + 1;
        //     word
        // });
        // // chain() will make the iterator significantly slower
        // // .chain(iter::once(unsafe {
        // //     data::WORDS.get_unchecked(*start.get()..)
        // // }));

        // memchr is as fast as std, but harder to work with
        let words = data::WORDS.split('\n');

        // let mut ac = AhoCorasick::builder();
        // ac.start_kind(StartKind::Anchored)
        //     .match_kind(MatchKind::LeftmostLongest);
        // let ac = match (kana, word) {
        //     (true, true) => ac.build(data::kana::HEPBURN_KANAS.iter().cloned().chain(words)),
        //     (true, false) => ac.build(data::kana::HEPBURN_KANAS),
        //     (false, true) => ac.build(words),
        //     (false, false) => ac.build::<_, &str>([]),
        // }
        // .unwrap();

        let ac =
            CharwiseDoubleArrayAhoCorasickBuilder::new().match_kind(MatchKind::LeftmostLongest);
        let ac = match (kana, word) {
            (true, true) => ac.build(data::kana::HEPBURN_KANAS.iter().cloned().chain(words)),
            (true, false) => ac.build(data::kana::HEPBURN_KANAS),
            (false, true) => ac.build(words),
            (false, false) => ac.build([] as [&str; 0]),
        }
        .unwrap();

        Self { ac, kanji }
    }

    /// ```
    /// use ib_romaji::HepburnRomanizer;
    ///
    /// assert_eq!(HepburnRomanizer::builder().kana(true).build().romanize_kana("あ"), Some((3, "a")));
    /// ```
    /// TODO: Iter
    pub fn romanize_kana<S: ?Sized + AsRef<str>>(&self, s: &S) -> Option<(usize, &'static str)> {
        let s = s.as_ref();
        let s = &s[..floor_char_boundary(s, data::kana::KANA_MAX_LEN)];
        // let m = self.ac.find(Input::new(s).anchored(Anchored::Yes))?;
        // let pattern = m.pattern().as_usize();
        let m = self
            .ac
            .leftmost_find_iter(s)
            .next()
            .filter(|m| m.start() == 0)?;
        let pattern = m.value() as usize;
        let len = m.end() - m.start();
        data::kana::HEPBURN_ROMAJIS
            .get(pattern)
            .map(|&romaji| (len, romaji))
    }

    pub fn romanize_kana_str<S: ?Sized + AsRef<str>>(&self, s: &S) -> Option<(usize, String)> {
        let s = s.as_ref();
        let mut len = 0;
        let mut buf = String::new();
        while let Some((l, romaji)) = self.romanize_kana(&s[len..]).or_else(|| {
            if s[len..].starts_with("、") {
                Some((3, "、"))
            } else {
                None
            }
        }) {
            len += l;
            buf.push_str(romaji);
            if len >= s.len() {
                return Some((len, buf));
            }
        }
        if len == 0 { None } else { Some((len, buf)) }
    }

    pub fn romanize_kana_str_all<S: ?Sized + AsRef<str>>(&self, s: &S) -> Option<String> {
        let s = s.as_ref();
        match self.romanize_kana_str(s) {
            Some((len, buf)) if len == s.len() => Some(buf),
            _ => None,
        }
    }

    pub fn romanize_and_try_for_each<S: ?Sized + AsRef<str>, T>(
        &self,
        s: &S,
        mut f: impl FnMut(usize, &'static str) -> Option<T>,
    ) -> Option<T> {
        let s = s.as_ref();
        let s = &s[..floor_char_boundary(s, data::WORD_MAX_LEN)];

        // self.ac.find(Input::new(s).anchored(Anchored::Yes))
        if let Some(m) = self
            .ac
            .leftmost_find_iter(s)
            .next()
            .filter(|m| m.start() == 0)
        {
            // let pattern = m.pattern().as_usize();
            let pattern = m.value() as usize;
            let len = m.end() - m.start();
            if pattern < data::kana::HEPBURN_ROMAJIS.len() {
                let romaji = data::kana::HEPBURN_ROMAJIS[pattern];
                if let Some(result) = f(len, romaji) {
                    return Some(result);
                }
            } else if pattern < data::kana::HEPBURN_ROMAJIS.len() + data::WORD_ROMAJIS.len() {
                // TODO: Binary search
                for romaji in data::WORD_ROMAJIS[pattern - data::kana::HEPBURN_ROMAJIS.len()] {
                    if let Some(result) = f(len, romaji) {
                        return Some(result);
                    }
                }
            }
        }

        if self.kanji {
            // let s = unsafe { str::from_utf8_unchecked(s) };
            if let Some(kanji) = s.chars().next() {
                // TODO: Binary search
                for romaji in data::kanji_romajis(kanji) {
                    // TODO: Always 3?
                    if let Some(result) = f(kanji.len_utf8(), romaji) {
                        return Some(result);
                    }
                }
            }
        }

        None
    }

    pub fn romanize_vec<S: ?Sized + AsRef<str>>(&self, s: &S) -> Vec<(usize, &'static str)> {
        let mut results = Vec::new();
        self.romanize_and_try_for_each(s, |len, romaji| {
            results.push((len, romaji));
            None::<()>
        });
        results
    }

    pub fn is_romanizable<S: ?Sized + AsRef<str>>(&self, s: &S) -> bool {
        let s = s.as_ref();
        if s.is_empty() {
            return true;
        }
        self.romanize_and_try_for_each(s, |len, _| self.is_romanizable(&s[len..]).then_some(()))
            .is_some()
    }

    pub fn is_romanizable_to<S: ?Sized + AsRef<str>>(&self, s: &S, romaji: &S) -> bool {
        let s = s.as_ref();
        let romaji = romaji.as_ref();
        if s.is_empty() {
            return romaji.is_empty();
        }
        self.romanize_and_try_for_each(s, |len, word_romaji| {
            self.is_romanizable_to(&s[len..], romaji.strip_prefix(word_romaji)?)
                .then_some(())
        })
        .is_some()
    }
}

impl Default for HepburnRomanizer {
    fn default() -> Self {
        Self::builder().kana(true).kanji(true).word(true).build()
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, io::Write};

    use indexmap::IndexSet;

    use super::*;

    #[test]
    fn kana_max_len() {
        let max_len = data::kana::HEPBURN_KANAS
            .iter()
            .inspect(|kana| {
                if kana.len() == data::kana::KANA_MAX_LEN {
                    println!("{}", kana);
                }
            })
            .map(|s| s.len())
            .max()
            .unwrap();
        assert_eq!(data::kana::KANA_MAX_LEN, max_len);
    }

    #[test]
    fn kana() {
        let data = HepburnRomanizer::builder().kana(true).build();
        assert_eq!(data.romanize_kana("は"), Some((3, "ha")));
        assert_eq!(data.romanize_kana("ハハハ"), Some((3, "ha")));
        assert_eq!(data.romanize_kana("ジョジョ"), Some((6, "jo")));
        assert_eq!(data.romanize_kana("って"), Some((6, "tte")));
        assert_eq!(data.romanize_kana("日は"), None);
    }

    #[test]
    fn kana_str() {
        let data = HepburnRomanizer::builder().kana(true).build();
        assert_eq!(data.romanize_kana_str("は"), Some((3, "ha".into())));
        assert_eq!(data.romanize_kana_str("ハハハ"), Some((9, "hahaha".into())));
        assert_eq!(
            data.romanize_kana_str("ジョジョ"),
            Some((12, "jojo".into()))
        );
        assert_eq!(data.romanize_kana_str("って"), Some((6, "tte".into())));
        assert_eq!(data.romanize_kana_str("日は"), None);
    }

    #[test]
    fn is_romanizable_to() {
        let data = HepburnRomanizer::builder().kana(true).kanji(true).build();
        assert!(data.is_romanizable_to("は", "ha"));
        assert!(data.is_romanizable_to("ハハハ", "hahaha"));
        assert!(data.is_romanizable_to("ジョジョ", "jojo"));
        assert!(data.is_romanizable_to("って", "tte"));
        assert!(data.is_romanizable_to("日は", "hiha"));
        assert!(data.is_romanizable_to("日は", "kusaha"));
        assert!(!data.is_romanizable_to("今日", "kyou"));
        assert!(data.is_romanizable_to("今日", "imakusa"));
    }

    #[ignore]
    #[test]
    fn codegen_kanji() {
        let romanizer = HepburnRomanizer::builder().kana(true).build();

        let mut dup_count = 0;

        let kanjidic = fs::read_to_string("data/kanjidic.csv").unwrap();
        let mut out_kanjis = fs::File::create("src/data/kanjis.rs").unwrap();
        writeln!(out_kanjis, "match kanji {{").unwrap();
        let mut range = 0;
        for (_i, line) in kanjidic.lines().enumerate() {
            let (kanji, kanas) = match line.split_once('\t') {
                Some(v) => v,
                None => continue,
            };

            write!(out_kanjis, "'{kanji}'=>").unwrap();

            let kanas_count = kanas.split('\t').count();
            let mut kanas_set: IndexSet<String> = kanas
                .split('\t')
                .map(|kana| match romanizer.romanize_kana_str_all(kana) {
                    Some(romaji) => format!("\"{}\"", romaji),
                    None => {
                        println!("Failed to romanize kana: {kana}");
                        kana.into()
                    }
                })
                .collect();
            kanas_set.sort_unstable();
            if kanas_set.len() != kanas_count {
                // println!("Duplicated romajis: {kanji}\t{kanas}");
                dup_count += 1;
            }

            write!(
                out_kanjis,
                "&[{}],",
                kanas_set.into_iter().collect::<Vec<_>>().join(",")
            )
            .unwrap();

            // (i + 1) % 8 == 0
            // Natural align
            let c = kanji.chars().next().unwrap() as u32;
            if c / 10 != range {
                range = c / 10;
                out_kanjis.write_all(b"\n").unwrap();
            }
        }
        write!(out_kanjis, "_ => &[]\n}}").unwrap();

        println!("Kanjis with duplicated romajis: {dup_count}");
    }

    /// `codegen_kanji()` should be run first.
    ///
    /// `cargo test --package ib-romaji --lib -r -- tests::codegen_word --exact --no-capture --ignored > data/word.txt`
    #[ignore]
    #[test]
    fn codegen_word() {
        let romanizer = HepburnRomanizer::builder().kana(true).build();
        let kanji_romanizer = HepburnRomanizer::builder().kana(true).kanji(true).build();

        let mut dup_count = 0;
        let mut romanizable_count = 0;
        let mut partial_romanizable_count = 0;
        let mut diff_romanizable_count = 0;
        let mut unromanizable_count = 0;
        let mut max_len = 0;

        let jmdict = fs::read_to_string("data/jmdict.csv").unwrap();
        let mut out_words = fs::File::create("src/data/words.in.txt").unwrap();
        let mut out_kanas = fs::File::create("src/data/word_kanas.rs").unwrap();
        // writeln!(out_words, "&[").unwrap();
        // writeln!(out_words, "\"").unwrap();
        // let end = jmdict.lines().count() - 1;
        writeln!(out_kanas, "&[").unwrap();
        // let mut c = 0;
        let mut range = 0;
        let mut range_c = 0;
        let mut range_2 = 0;
        for (i, line) in jmdict.lines().enumerate() {
            let (word, kanas) = match line.split_once('\t') {
                Some(v) => v,
                None => continue,
            };

            let kanas_count = kanas.split('\t').count();
            let kanas_set: IndexSet<String> = kanas
                .split('\t')
                .map(|kana| match romanizer.romanize_kana_str_all(kana) {
                    // format!("\"{}\"", romaji)
                    Some(romaji) => romaji,
                    None => {
                        println!("Failed to romanize kana: {kana}");
                        kana.into()
                    }
                })
                .collect();
            if kanas_set.len() != kanas_count {
                // println!("Duplicated romajis: {kanji}\t{kanas}");
                dup_count += 1;
            }

            // Filter out ordinary words
            // Source file: 2.52+3.59=6.11 MiB -> 1.07+1.45=2.52 MiB
            // Binary: -10.01 MiB
            // TODO: What if the dependent word is in words?
            let mut romajis = if kanji_romanizer.is_romanizable(word) {
                let romajis = kanas_set
                    .iter()
                    .cloned()
                    .filter(|romaji| !kanji_romanizer.is_romanizable_to(word, romaji))
                    .collect::<Vec<_>>();
                if romajis.len() != kanas_set.len() {
                    if romajis.is_empty() {
                        // println!("romanizable: {word}");
                        romanizable_count += 1;
                        continue;
                    }
                    println!(
                        "partial: {word} -{} {kanas_set:?} -> {romajis:?}",
                        kanas_set.len() - romajis.len()
                    );
                    partial_romanizable_count += 1;
                } else {
                    println!("diff: {word} {kanas_set:?}");
                    diff_romanizable_count += 1;
                }
                romajis
            } else {
                println!("un: {word}");
                unromanizable_count += 1;
                kanas_set.into_iter().collect()
            };
            romajis.sort_unstable();

            if word.len() > max_len {
                max_len = word.len();
            }

            // write!(out_words, "\"{kanji}\",").unwrap();
            // if i != end {
            //     write!(out_words, "{word}\n").unwrap();
            // } else {
            //     write!(out_words, "{word}").unwrap();
            // }
            if i == 0 {
                write!(out_words, "{word}").unwrap();
            } else {
                write!(out_words, "\n{word}").unwrap();
            }

            // i != 0 && (c + 1) % 8 == 0
            // Natural align
            let ch = word.chars().next().unwrap() as u32;
            let ch2 = word.chars().nth(1).unwrap_or_default() as u32;
            if ch / 100 != range || range_c > 10 && ch2 / 100 != range_2 {
                if ch / 100 != range {
                    range = ch / 100;
                    range_c = 0;
                }
                range_2 = ch2 / 100;
                if i != 0 {
                    // out_words.write_all(b"\n").unwrap();
                    // out_words.write_all(b"\\\n").unwrap();
                    out_kanas.write_all(b"\n").unwrap();
                }
            } else {
                range_c += 1;
            }

            write!(
                out_kanas,
                "&[{}],",
                romajis
                    .into_iter()
                    .map(|romaji| format!("\"{}\"", romaji))
                    .collect::<Vec<_>>()
                    .join(",")
            )
            .unwrap();

            // c += 1;
        }
        // write!(out_words, "\n]").unwrap();
        // write!(out_words, "\\\n\"").unwrap();
        write!(out_kanas, "\n]").unwrap();

        println!("Words with duplicated romajis: {dup_count}");
        println!();
        println!("Romanizable words: {romanizable_count}");
        println!("Partial romanizable words: {partial_romanizable_count}");
        println!("Different romanizable words: {diff_romanizable_count}");
        println!("Unromanizable words: {unromanizable_count}");
        println!();
        println!("Max word length: {max_len}");
        assert_eq!(data::WORD_MAX_LEN, max_len);
    }

    #[test]
    fn kanji() {
        assert_eq!(
            data::kanji_romajis('日'),
            [
                "a", "aki", "bi", "chi", "he", "hi", "iru", "jitsu", "ka", "kou", "ku", "kusa",
                "nchi", "ni", "nichi", "nitsu", "su", "tachi"
            ]
        );

        let data = HepburnRomanizer::builder().kana(true).kanji(true).build();
        assert_eq!(data.romanize_vec("は"), vec![(3, "ha")]);
        assert_eq!(data.romanize_vec("ハハハ"), vec![(3, "ha")]);
        assert_eq!(data.romanize_vec("ジョジョ"), vec![(6, "jo")]);
        assert_eq!(data.romanize_vec("って"), vec![(6, "tte")]);
        assert_eq!(
            data.romanize_vec("日は"),
            [
                "a", "aki", "bi", "chi", "he", "hi", "iru", "jitsu", "ka", "kou", "ku", "kusa",
                "nchi", "ni", "nichi", "nitsu", "su", "tachi"
            ]
            .map(|romaji| (3, romaji))
        );
        assert_eq!(
            data.romanize_vec("今日"),
            vec![(3, "ima"), (3, "kin"), (3, "kon"), (3, "na")]
        );
    }

    #[test]
    fn word() {
        let data = HepburnRomanizer::builder().kana(true).word(true).build();
        assert_eq!(data.romanize_vec("は"), vec![(3, "ha")]);
        assert_eq!(data.romanize_vec("ハハハ"), vec![(3, "ha")]);
        assert_eq!(data.romanize_vec("ジョジョ"), vec![(6, "jo")]);
        assert_eq!(data.romanize_vec("って"), vec![(6, "tte")]);
        assert_eq!(data.romanize_vec("日は"), vec![]);
        assert_eq!(data.romanize_vec("今日"), vec![(6, "kyou")]);

        let data = HepburnRomanizer::builder()
            .kana(true)
            .kanji(true)
            .word(true)
            .build();
        assert_eq!(
            data.romanize_vec("今日"),
            vec![(6, "kyou"), (3, "ima"), (3, "kin"), (3, "kon"), (3, "na")]
        );
    }
}
