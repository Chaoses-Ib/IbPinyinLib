# IbPinyinLib.Rust
[![crates.io](https://img.shields.io/crates/v/ib-pinyin.svg)](https://crates.io/crates/ib-pinyin)
[![Documentation](https://docs.rs/ib-pinyin/badge.svg)](https://docs.rs/ib-pinyin)
[![License](https://img.shields.io/crates/l/ib-pinyin.svg)](../LICENSE.txt)

## Usage
```rust
use ib_pinyin::{matcher::PinyinMatcher, pinyin::PinyinNotation};

let matcher = PinyinMatcher::builder("pysousuoeve")
    .pinyin_notations(PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter)
    .build();
assert!(matcher.is_match("拼音搜索Everything"));
```

## Testing
```sh
cargo hack test --feature-powerset
```

## See also
- [rust-pinyin: 汉字转拼音](https://github.com/mozillazg/rust-pinyin)
- [samlink/rust-pinyin: Chinese pinyin initials in rust](https://github.com/samlink/rust_pinyin)