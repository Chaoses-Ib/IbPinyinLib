#pragma once
#include <string>

namespace pinyin
{
#ifndef IB_PINYIN_ENCODING
#error IB_PINYIN_ENCODING must be defined before including Pinyin.hpp
#elif IB_PINYIN_ENCODING == 8
    using Char = char;
    using String = std::string;
    using StringView = std::string_view;
#define IB_PINYIN_LITERAL(s) u8##s
#elif IB_PINYIN_ENCODING == 16
    using Char = char16_t;
    using String = std::u16string;
    using StringView = std::u16string_view;
#define IB_PINYIN_LITERAL(s) u##s
#elif IB_PINYIN_ENCODING == 32
    using Char = char32_t;
    using String = std::u32string;
    using StringView = std::u32string_view;
#define IB_PINYIN_LITERAL U##s
#endif
    char32_t read_char32(const Char* str, int* length);

    using PinyinFlagValue = int;
    struct PinyinFlag {
        using T = const PinyinFlagValue;
        static T Pinyin = 1;
        static T PinyinAscii = 2;
        static T PinyinAsciiDigit = 4;
        static T DoublePinyinXiaohe = 8;
        static T Initial = 16;
    };
    struct Pinyin {
        const StringView pinyin;
        const StringView pinyin_ascii;
        const StringView pinyin_ascii_digit;
        const StringView double_pinyin_xiaohe;
        const Char initial;
    
        Pinyin(StringView pinyin, StringView pinyin_ascii_digit, StringView double_pinyin_xiaohe);
    };

#pragma pack(push, 2)
    template <uint16_t N>
    struct PinyinCombination {
        uint16_t n = N;
        uint16_t pinyin[N];
    };

    struct PinyinRange {
        char32_t begin;
        char32_t end;
        uint16_t* table;
    };
#pragma pack(pop)

    extern Pinyin pinyins[1514];
    extern PinyinCombination<10> pinyin_combinations[1104];
    extern PinyinRange pinyin_ranges[7];


    // return 0xFFFF if failed
    uint16_t get_pinyin_index(char32_t hanzi);

    // return 0 if no match
    size_t match_pinyin(StringView pinyin, char32_t hanzi, PinyinFlagValue flags);
    

    class Matcher {
        [[deprecated]]
        Matcher(PinyinFlagValue flags);

    protected:
        PinyinFlagValue flags;
    };
}