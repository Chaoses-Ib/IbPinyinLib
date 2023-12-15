# IbPinyinLib
## 语言
### Rust
```rust
use ib_pinyin::{matcher::PinyinMatcher, pinyin::PinyinNotation};

let matcher = PinyinMatcher::builder("pysousuoeve")
    .pinyin_notations(PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter)
    .build();
assert!(matcher.is_match("拼音搜索Everything"));
```

### C
```c
#include <ib_pinyin/ib_pinyin.h>
#include <ib_pinyin/notation.h>

bool is_match = ib_pinyin_is_match_u8c(u8"pysousuoeve", u8"拼音搜索Everything", PINYIN_NOTATION_ASCII_FIRST_LETTER | PINYIN_NOTATION_ASCII);
```

### C++
[原实现](C++/README.md)（停止维护）

## 相关项目
- [IbEverythingExt: Everything 拼音搜索、快速选择扩展](https://github.com/Chaoses-Ib/IbEverythingExt)
- [pinyin-data: 汉字拼音数据](https://github.com/Chaoses-Ib/pinyin-data)

其它拼音库：
- [rust-pinyin: 汉字转拼音](https://github.com/mozillazg/rust-pinyin)
- [samlink/rust-pinyin: Chinese pinyin initials in rust](https://github.com/samlink/rust_pinyin)