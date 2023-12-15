#pragma once
#include <stdint.h>

/// All pinyin notations are in lower case (`py.to_lowercase() == py`).
typedef uint32_t PinyinNotation;

/// e.g. "pīn", "yīn"
#define PINYIN_NOTATION_UNICODE 0x8

/// 全拼
///
/// e.g. "pin", "yin"
///
/// See [全拼](https://zh.wikipedia.org/wiki/全拼) for details.
#define PINYIN_NOTATION_ASCII 0x2

/// 带声调全拼
///
/// The tone digit is in `1..=5`. See [tones](https://en.wikipedia.org/wiki/Pinyin#Tones) for details.
///
/// e.g. "pin1", "yin1"
#define PINYIN_NOTATION_ASCII_TONE 0x4

/// 简拼
///
/// e.g. "p", "y"
///
/// See [简拼](https://zh.wikipedia.org/wiki/简拼) for details.
#define PINYIN_NOTATION_ASCII_FIRST_LETTER 0x1

/// 智能 ABC 双拼
///
/// See [智能ABC输入法](https://zh.wikipedia.org/wiki/智能ABC输入法#双拼方案) for details.
#define PINYIN_NOTATION_DILETTER_ABC 0x10

/// 拼音加加双拼
///
/// See [拼音加加](https://zh.wikipedia.org/wiki/拼音加加#双拼方案) for details.
#define PINYIN_NOTATION_DiletterJiajia 0x20

/// 微软双拼
///
/// See [微软拼音输入法](https://zh.wikipedia.org/wiki/微软拼音输入法#双拼方案) for details.
#define PINYIN_NOTATION_DiletterMicrosoft 0x40

/// 华宇双拼（紫光双拼）
///
/// See [华宇拼音输入法](https://zh.wikipedia.org/wiki/华宇拼音输入法#双拼方案) for details.
#define PINYIN_NOTATION_DiletterThunisoft 0x80

/// 小鹤双拼
///
/// See [小鹤双拼](https://flypy.com/) for details.
#define PINYIN_NOTATION_DiletterXiaohe 0x100

/// 自然码双拼
///
/// See [自然码](https://zh.wikipedia.org/zh-cn/自然码) for details.
#define PINYIN_NOTATION_DiletterZrm 0x200