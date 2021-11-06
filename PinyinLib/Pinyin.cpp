#include "pch.h"
#include "Pinyin.hpp"

namespace pinyin {
#if IB_PINYIN_ENCODING == 8
    char32_t read_char32(const Char* str, int* length) {
        char c = str[0];
        switch (*length = 1 + ((c & 0b11000000) == 0b11000000) + ((c & 0b11100000) == 0b11100000) + ((c & 0b11110000) == 0b11110000)) {
        case 1: return c;
        case 2: return (c & 0b11111) << 6 | (str[1] & 0b111111);
        case 3: return (c & 0b1111) << 12 | (str[1] & 0b111111) << 6 | (str[2] & 0b111111);
        case 4: return (c & 0b111) << 18 | (str[1] & 0b111111) << 12 | (str[2] & 0b111111) << 6 | (str[3] & 0b111111);
        }
    }
#elif IB_PINYIN_ENCODING == 16
    char32_t read_char32(const Char* str, int* length) {
        char16_t c = str[0];
        if (0xD800 <= c && c <= 0xDBFF) {
            *length = 2;
            return 0x10000 + ((c - 0xD800) << 10) + (str[1] - 0xDC00);
        } else {
            *length = 1;
            return c;
        }
    }
#elif IB_PINYIN_ENCODING == 32
    char32_t read_char32(const Char* str, int* length) {
        *length = 1;
        return str[0];
    }
#endif

    Pinyin::Pinyin(StringView pinyin, StringView pinyin_ascii_digit, StringView double_pinyin_xiaohe)
      : pinyin(pinyin),
        pinyin_ascii(pinyin_ascii_digit.substr(0, pinyin_ascii_digit.size() - 1)),
        pinyin_ascii_digit(pinyin_ascii_digit),
        double_pinyin_xiaohe(double_pinyin_xiaohe),
        initial(pinyin_ascii_digit[0]) {}

    uint16_t get_pinyin_index(char32_t hanzi) {
        for (PinyinRange range : pinyin_ranges) {
            if (range.begin <= hanzi && hanzi <= range.end) {
                return range.table[hanzi - range.begin];
            }
        }
        return 0xFFFF;
    }

    size_t match_pinyin(StringView string, char32_t hanzi, PinyinFlagValue flags) {
        auto starts_with = [](StringView s1, StringView s2) -> size_t {
            if (s1.rfind(s2, 0) == 0)
                return s2.size();
            else
                return 0;
        };
        auto match = [string, flags, &starts_with](Pinyin& pinyin) -> size_t {
            size_t size;
            if (flags & PinyinFlag::Pinyin) {
                if (size = starts_with(string, pinyin.pinyin))
                    return size;
            }
            if (flags & PinyinFlag::PinyinAsciiDigit) {
                if (size = starts_with(string, pinyin.pinyin_ascii_digit))
                    return size;
            }
            if (flags & PinyinFlag::PinyinAscii) {
                if (size = starts_with(string, pinyin.pinyin_ascii))
                    return size;
            }
            if (flags & PinyinFlag::DoublePinyinXiaohe) {
                if (size = starts_with(string, pinyin.double_pinyin_xiaohe))
                    return size;
            }
            if (flags & PinyinFlag::Initial) {
                if (string.size() && string[0] == pinyin.initial)
                    return 1;
            }
            return 0;
        };

        uint16_t index = get_pinyin_index(hanzi);
        if (index == 0xFFFF)
            return 0;

        size_t size;
        if (index < std::size(pinyins)) {
            if (size = match(pinyins[index]))
                return size;
        } else {
            index -= std::size(pinyins);
            auto comb = pinyin_combinations[index];
            for (uint16_t i = 0; i < comb.n; i++) {
                if (size = match(pinyins[comb.pinyin[i]]))
                    return size;
            }
        }
    
        return 0;
    }

    Matcher::Matcher(PinyinFlagValue flags) : flags(flags) {}
}