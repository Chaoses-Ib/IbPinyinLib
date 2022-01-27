#pragma once
#include <string>
#include <unordered_map>

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
#define IB_PINYIN_LITERAL(s) U##s
#endif
    char32_t read_char32(const Char* str, int* length);

    using PinyinFlagValue = int;
    struct PinyinFlag {
        using T = const PinyinFlagValue;
        static T Pinyin = 1;
        static T PinyinAsciiDigit = 2;
        static T PinyinAscii = 4;
        static T InitialLetter = 8;
        static T DoublePinyinXiaohe = 16;
    };
    struct Pinyin {
        const StringView pinyin;

        String pinyin_ascii_digit;
        StringView pinyin_ascii;
        Char initial_letter;
        
        String double_pinyin_xiaohe;
    
        Pinyin(StringView pinyin);

        // only Pinyin is initialized at startup, other PinyinFlag need to be initialized by calling init (and be detroyed by calling destroy)
        void init(PinyinFlagValue flags);
        // will not clear initial_letter
        void destroy();

        /*
        initials = {
            "b", "p", "m", "f",
            "d", "t", "n", "z", "c", "s", "l",
            "zh", "ch", "sh", "r",
            "j", "q", "x",
            "g", "k", "h",
            "y", "w"
        }
        finals = {
            "i", "u", "v",
            "e", "ie", "o", "uo", "ue", "ve",
            "a", "ia", "ua",
            "ei", "ui",
            "ai", "uai",
            "ou", "iu",
            "ao", "iao",
            "in", "un", "vn",
            "en",
            "an", "ian", "uan", "van",
            "ing",
            "ong", "iong",
            "eng",
            "ang", "iang", "uang",
            "er"
        }
        https://en.wikipedia.org/wiki/Pinyin
        */
        String convert(
            const std::unordered_map<StringView, StringView>& pinyin_map,
            const std::unordered_map<StringView, StringView>& initial_map,
            const std::unordered_map<StringView, StringView>& final_map) const;
        String to_pinyin_ascii_digit() const;
        String to_pinyin_ascii() const;
        Char to_initial_letter() const;
        String to_double_pinyin_xiaohe() const;
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

    // only Pinyin is initialized at startup, other PinyinFlag need to be initialized by calling init (and be detroyed by calling destroy)
    void init(PinyinFlagValue flags);
    // will not clear initial_letter
    void destroy();

    // return 0xFFFF if failed
    uint16_t get_pinyin_index(char32_t hanzi);

    // require InitialLetter to be initialized
    // return 0 if failed
    uint32_t get_initial_pinyin_letters(char32_t hanzi);

    // return 0 if no match
    size_t match_pinyin(char32_t hanzi, StringView pinyin, PinyinFlagValue flags);
    

    class Matcher {
        [[deprecated]]
        Matcher(PinyinFlagValue flags);

    protected:
        PinyinFlagValue flags;
    };
}