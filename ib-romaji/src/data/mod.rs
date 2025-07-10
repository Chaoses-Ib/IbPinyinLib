#![cfg_attr(rustfmt, rustfmt_skip)]

pub mod kana;

// pub static WORDS: &[&str] = &[];
pub static WORDS: &[&str] = include!("words.rs");

// pub static WORD_ROMAJIS: &[&[&str]] = &[&["onaji", "onajiku"], &["dou"]];
pub static WORD_ROMAJIS: &[&[&str]] = include!("word_kanas.rs");

pub fn kanji_romajis(kanji: char) -> &'static [&'static str] {
    include!("kanjis.rs")
}