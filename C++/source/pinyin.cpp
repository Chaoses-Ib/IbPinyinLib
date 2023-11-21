#include <IbPinyin/pinyin.hpp>
#include <algorithm>
#include <cassert>

#define LITERAL(s) IB_PINYIN_LITERAL(s)

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

    Pinyin::Pinyin(StringView pinyin) : pinyin(pinyin), initial_letter('\0') {}

    void Pinyin::init(PinyinFlagValue flags)
    {
        if (flags & PinyinFlag::PinyinAsciiDigit || flags & PinyinFlag::PinyinAscii) {
            pinyin_ascii_digit = to_pinyin_ascii_digit();
            pinyin_ascii = StringView(pinyin_ascii_digit).substr(0, pinyin_ascii_digit.size() - 1);
        }
        if (flags & PinyinFlag::InitialLetter)
            initial_letter = to_initial_letter();

        if (flags & PinyinFlag::DoublePinyinAbc)
            double_pinyin_abc = to_double_pinyin_abc();
        if (flags & PinyinFlag::DoublePinyinJiajia)
            double_pinyin_jiajia = to_double_pinyin_jiajia();
        if (flags & PinyinFlag::DoublePinyinMicrosoft)
            double_pinyin_microsoft = to_double_pinyin_microsoft();
        if (flags & PinyinFlag::DoublePinyinThunisoft)
            double_pinyin_thunisoft = to_double_pinyin_thunisoft();
        if (flags & PinyinFlag::DoublePinyinXiaohe)
            double_pinyin_xiaohe = to_double_pinyin_xiaohe();
        if (flags & PinyinFlag::DoublePinyinZrm)
            double_pinyin_zrm = to_double_pinyin_zrm();
    }

    void Pinyin::destroy()
    {
        pinyin_ascii_digit = {};
        pinyin_ascii = {};
        double_pinyin_xiaohe = {};
    }

    String Pinyin::convert(const std::unordered_map<StringView, StringView>& pinyin_map, const std::unordered_map<StringView, StringView>& initial_map, const std::unordered_map<StringView, StringView>& final_map) const
    {
        StringView ascii;
        String ascii_s;
        if (pinyin_ascii.empty()) {
            ascii_s = to_pinyin_ascii();
            ascii = ascii_s;
        }
        else
            ascii = pinyin_ascii;

        if (ascii == LITERAL("hm"))  // 噷
            ascii = LITERAL("hen");
        else if (ascii == LITERAL("hng"))  // 哼
            ascii = LITERAL("heng");
        else if (ascii == LITERAL("m"))  // 呒呣嘸
            ascii = LITERAL("mu");
        else if (ascii == LITERAL("n") || ascii == LITERAL("ng"))  // 唔嗯 㕶 𠮾
            ascii = LITERAL("en");
        
        if (auto it = pinyin_map.find(ascii); it != pinyin_map.end())
            return String(it->second);

        String result;
        if (ascii.size() >= 2) {
            StringView first_two = ascii.substr(0, 2);
            if (first_two == LITERAL("zh") || first_two == LITERAL("ch") || first_two == LITERAL("sh")) {
                auto it = initial_map.find(first_two);
                assert(it != initial_map.end());
                result = it->second;

                ascii = ascii.substr(2);
            }
            else
                goto single_initial;
        }
        else {
        single_initial:
            if (initial_map.size() == 3) {
                Char c = ascii[0];
                if (c != 'a' && c != 'e' && c != 'i' && c != 'o' && c != 'u' && c != 'v') {
                    result = c;
                    ascii = ascii.substr(1);
                } 
            }
            else {
                auto it = initial_map.find(ascii.substr(0, 1));
                assert(it != initial_map.end());
                result = it->second;

                ascii = ascii.substr(1);
            }
        }

        auto it = final_map.find(ascii);
        assert(it != final_map.end());
        result += it->second;

        return result;
    }

    String Pinyin::to_pinyin_ascii_digit() const
    {
        String ascii_digit = to_pinyin_ascii();

        static StringView t1[] = { LITERAL("ā"), LITERAL("ē"), LITERAL("ī"), LITERAL("ō"),
            LITERAL("ū"), LITERAL("ê̄") };
        static StringView t2[] = { LITERAL("á"), LITERAL("é"), LITERAL("ế"), LITERAL("í"), 
            LITERAL("ó"), LITERAL("ú"), LITERAL("ǘ"), LITERAL("ḿ"), LITERAL("ń") };
        static StringView t3[] = { LITERAL("ǎ"), LITERAL("ě"), LITERAL("ǐ"), LITERAL("ǒ"),
            LITERAL("ǔ"), LITERAL("ǚ"), LITERAL("ň"), LITERAL("ê̌") };
        static StringView t4[] = { LITERAL("à"), LITERAL("è"), LITERAL("ề"), LITERAL("ì"),
            LITERAL("ò"), LITERAL("ù"), LITERAL("ǜ"), LITERAL("ǹ"), LITERAL("m̀") };

        auto test = [this](StringView* t, size_t n) {
            for (size_t i = 0; i < n; i++) {
                if (std::search(pinyin.begin(), pinyin.end(), t[i].begin(), t[i].end()) != pinyin.end())
                    return true;
            }
            return false;
        };
        if (test(t1, std::size(t1))) return ascii_digit + Char('1');
        if (test(t2, std::size(t2))) return ascii_digit + Char('2');
        if (test(t3, std::size(t3))) return ascii_digit + Char('3');
        if (test(t4, std::size(t4))) return ascii_digit + Char('4');
        return ascii_digit + Char('5');
    }

    String Pinyin::to_pinyin_ascii() const
    {
        StringView py = pinyin;
        String ascii;
        while (py.size()) {
            Char c = py[0];
            if ('a' <= c && c <= 'z') {
                int length;
                if (c == 'm' && py.size() > 1 && read_char32(py.data() + 1, &length) == U'̀') {
                    ascii.push_back('m');
                    py = py.substr(1 + length);
                    continue;
                }
                ascii.push_back(c);
                py = py.substr(1);
            }
            else {
                static StringView t_a[] = { LITERAL("ā"), LITERAL("á"), LITERAL("ǎ"), LITERAL("à") };
                static StringView t_e[] = { LITERAL("ē"), LITERAL("é"), LITERAL("ě"), LITERAL("è"), LITERAL("ế"), LITERAL("ề"), LITERAL("ê̄"), LITERAL("ê̌") };
                static StringView t_i[] = { LITERAL("ī"), LITERAL("í"), LITERAL("ǐ"), LITERAL("ì") };
                static StringView t_o[] = { LITERAL("ō"), LITERAL("ó"), LITERAL("ǒ"), LITERAL("ò") };
                static StringView t_u[] = { LITERAL("ū"), LITERAL("ú"), LITERAL("ǔ"), LITERAL("ù") };
                static StringView t_v[] = { LITERAL("ü"), LITERAL("ǘ"), LITERAL("ǚ"), LITERAL("ǜ") };
                static StringView t_n[] = { LITERAL("ń"), LITERAL("ň"), LITERAL("ǹ") };
                static StringView t_m[] = { LITERAL("ḿ") };  // LITERAL("m̀")

                auto test = [&py, &ascii](StringView* t, size_t n, Char c) {
                    for (size_t i = 0; i < n; i++) {
                        if (py.substr(0, t[i].size()) == t[i]) {
                            ascii.push_back(c);
                            py = py.substr(t[i].size());
                            return true;
                        }
                    }
                    return false;
                };
                if (test(t_a, std::size(t_a), 'a'));
                else if (test(t_e, std::size(t_e), 'e'));
                else if (test(t_i, std::size(t_i), 'i'));
                else if (test(t_o, std::size(t_o), 'o'));
                else if (test(t_u, std::size(t_u), 'u'));
                else if (test(t_v, std::size(t_v), 'v'));
                else if (test(t_n, std::size(t_n), 'n'));
                else if (test(t_m, std::size(t_m), 'm'));
                else
                    assert(false);
            }
        }
        return ascii;
    }

    Char Pinyin::to_initial_letter() const
    {
        if (pinyin_ascii.size())
            return pinyin_ascii[0];

        //#TODO: could be optimized
        String ascii = to_pinyin_ascii();
        return ascii[0];
    }

    void init(PinyinFlagValue flags)
    {
        for (Pinyin& py : pinyins)
            py.init(flags);
    }

    void destroy()
    {
        for (Pinyin& py : pinyins)
            py.destroy();
    }

    uint16_t get_pinyin_index(char32_t hanzi) {
        for (PinyinRange range : pinyin_ranges) {
            if (range.begin <= hanzi && hanzi <= range.end) {
                return range.table[hanzi - range.begin];
            }
        }
        return 0xFFFF;
    }
    
    uint32_t get_initial_pinyin_letters(char32_t hanzi)
    {
        uint32_t initials;

        uint16_t index = get_pinyin_index(hanzi);
        if (index == 0xFFFF)
            return 0;

        size_t size;
        if (index < std::size(pinyins)) {
            initials = 1 << (pinyins[index].initial_letter - 'a');
        }
        else {
            initials = 0;
            index -= std::size(pinyins);
            auto comb = pinyin_combinations[index];
            for (uint16_t i = 0; i < comb.n; i++) {
                initials |= 1 << (pinyins[comb.pinyin[i]].initial_letter - 'a');
            }
        }

        return initials;
    }

    size_t match_pinyin(char32_t hanzi, StringView string, PinyinFlagValue flags) {
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

            if (flags & PinyinFlag::DoublePinyinAbc) {
                if (size = starts_with(string, pinyin.double_pinyin_abc))
                    return size;
            }
            if (flags & PinyinFlag::DoublePinyinJiajia) {
                if (size = starts_with(string, pinyin.double_pinyin_jiajia))
                    return size;
            }
            if (flags & PinyinFlag::DoublePinyinMicrosoft) {
                if (size = starts_with(string, pinyin.double_pinyin_microsoft))
                    return size;
            }
            if (flags & PinyinFlag::DoublePinyinThunisoft) {
                if (size = starts_with(string, pinyin.double_pinyin_thunisoft))
                    return size;
            }
            if (flags & PinyinFlag::DoublePinyinXiaohe) {
                if (size = starts_with(string, pinyin.double_pinyin_xiaohe))
                    return size;
            }
            if (flags & PinyinFlag::DoublePinyinZrm) {
                if (size = starts_with(string, pinyin.double_pinyin_zrm))
                    return size;
            }

            if (flags & PinyinFlag::InitialLetter) {
                if (string.size() && string[0] == pinyin.initial_letter)
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