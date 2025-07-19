#![cfg_attr(rustfmt, rustfmt_skip)]
use core::ops::Range;

pub mod kana;

pub const KANJI_MIN_LEN: usize = 2;
pub const KANJI_MAX_LEN: usize = 4;
pub const KANJI_LEN: Range<usize> = KANJI_MIN_LEN..KANJI_MAX_LEN+1;

/// i.e. 娍: suraritoshitemimeyoi (すらりとしてみめよい)
pub const KANJI_ROMAJI_MAX_LEN: usize = 22;

/// i.e. 身体髪膚これを父母に受くあえて毀傷せざるは孝の始めなり
pub const WORD_MAX_LEN: usize = 81;

/// i.e.
/// - 身体髪膚これを父母に受くあえて毀傷せざるは孝の始め?なり: shintaihappukorewofuboniukuaetekishousezaruhakounohajimenari
/// - 山中の賊を破るは易く心中の賊を破るは難し: sanchuunozokuwoyaburuhayasukushinchuunozokuwoyaburuhakatashi
pub const WORD_ROMAJI_MAX_LEN: usize = 60;

// pub static WORDS: &[&str] = &[];
// pub static WORDS: &[&str] = include!("words.rs");
pub(crate) static WORDS: &str = include_str!("words.in.txt");

// pub static WORD_ROMAJIS: &[&[&str]] = &[&["onaji", "onajiku"], &["dou"]];
pub(crate) static WORD_ROMAJIS: &[&[&str]] = include!("word_kanas.rs");

pub(crate) fn kanji_romajis(kanji: char) -> &'static [&'static str] {
    include!("kanjis.rs")
}