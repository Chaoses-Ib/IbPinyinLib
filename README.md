# IbPinyinLib
[![License](https://img.shields.io/crates/l/ib-pinyin.svg)](LICENSE.txt)

一个高性能 Rust 拼音匹配库。

- 支持以下拼音编码方案：
  - 简拼（“py”）
  - 全拼（“pinyin”）
  - 带声调全拼（“pin1yin1”）
  - Unicode（“pīnyīn”）
  - 智能 ABC 双拼
  - 拼音加加双拼
  - 微软双拼
  - 华宇双拼（紫光双拼）
  - 小鹤双拼
  - 自然码双拼
- 支持多音字。
- 支持混合匹配多种拼音编码方案，默认匹配简拼和全拼。
- 默认小写字母匹配拼音或字母，大写字母只匹配字母。
- 支持 Unicode 辅助平面汉字。

## 支持语言
### [Rust](src)
[![crates.io](https://img.shields.io/crates/v/ib-pinyin.svg)](https://crates.io/crates/ib-pinyin)
[![Documentation](https://docs.rs/ib-pinyin/badge.svg)](https://docs.rs/ib-pinyin)

```rust
use ib_pinyin::{matcher::PinyinMatcher, pinyin::PinyinNotation};

let matcher = PinyinMatcher::builder("pysousuoeve")
    .pinyin_notations(PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter)
    .build();
assert!(matcher.is_match("拼音搜索Everything"));
```

### [C](bindings/c)
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

### C++
[原实现](C++)（停止维护）

### [AutoHotkey v2](bindings/ahk2)
```ahk
#Include <IbPinyin>

IsMatch := IbPinyin_IsMatch("pysousuoeve", "拼音搜索Everything", IbPinyin_AsciiFirstLetter | IbPinyin_Ascii)

是否匹配 := 拼音_匹配("pysousuoeve", "拼音搜索Everything")
是否匹配 := 拼音_匹配("pysousuoeve", "拼音搜索Everything", 拼音_简拼 | 拼音_全拼)
```
[下载](https://github.com/Chaoses-Ib/IbPinyinLib/releases)

## 相关项目
- [IbEverythingExt: Everything 拼音搜索、快速选择扩展](https://github.com/Chaoses-Ib/IbEverythingExt)
- [pinyin-data: 汉字拼音数据](https://github.com/Chaoses-Ib/pinyin-data)