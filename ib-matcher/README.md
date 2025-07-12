# ib-matcher
[![crates.io](https://img.shields.io/crates/v/ib-matcher.svg)](https://crates.io/crates/ib-matcher)
[![Documentation](https://docs.rs/ib-matcher/badge.svg)](https://docs.rs/ib-matcher)
[![License](https://img.shields.io/crates/l/ib-matcher.svg)](LICENSE.txt)

A multilingual and fast string matcher, supports 拼音匹配 (Chinese pinyin match) and ローマ字検索 (Japanese romaji match).

## Usage
```rust
//! cargo add ib-matcher --features pinyin,romaji

use ib_matcher::{
    matcher::{IbMatcher, PinyinMatchConfig, RomajiMatchConfig},
    pinyin::PinyinNotation,
};

let matcher = IbMatcher::builder("pysousuoeve")
    .pinyin(PinyinMatchConfig::notations(
        PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter,
    ))
    .build();
assert!(matcher.is_match("拼音搜索Everything"));

let matcher = IbMatcher::builder("konosuba")
    .romaji(RomajiMatchConfig::default())
    .is_pattern_partial(true)
    .build();
assert!(matcher.is_match("この素晴らしい世界に祝福を"));
```

## Test
```sh
cargo build
cargo test --features pinyin,romaji
```
