# IbPinyinLib.C
## Usage
```c
#include <ib_pinyin/ib_pinyin.h>
#include <ib_pinyin/notation.h>

// UTF-8
bool is_match = ib_pinyin_is_match_u8c(u8"pysousuoeve", u8"拼音搜索Everything", PINYIN_NOTATION_ASCII_FIRST_LETTER | PINYIN_NOTATION_ASCII);

// UTF-16
bool is_match = ib_pinyin_is_match_u16c(u"pysousuoeve", u"拼音搜索Everything", PINYIN_NOTATION_ASCII_FIRST_LETTER | PINYIN_NOTATION_ASCII);

// UTF-32
bool is_match = ib_pinyin_is_match_u32c(U"pysousuoeve", U"拼音搜索Everything", PINYIN_NOTATION_ASCII_FIRST_LETTER | PINYIN_NOTATION_ASCII);
```

## Build
```sh
diplomat-tool c bindings/c/include/ib_pinyin -e bindings/c/src/lib.rs
```

Manually update: `notation.h`