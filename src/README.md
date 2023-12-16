# IbPinyinLib.Rust
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