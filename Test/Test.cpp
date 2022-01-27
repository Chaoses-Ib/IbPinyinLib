#define BOOST_TEST_MODULE Test
#include <boost/test/unit_test.hpp>

#define IB_PINYIN_ENCODING 32
#include "../PinyinLib/Pinyin.hpp"

BOOST_AUTO_TEST_CASE(MatchPinyin) {
    pinyin::PinyinFlagValue flags = pinyin::PinyinFlag::Pinyin
        | pinyin::PinyinFlag::PinyinAsciiDigit | pinyin::PinyinFlag::PinyinAscii | pinyin::PinyinFlag::InitialLetter
        | pinyin::PinyinFlag::DoublePinyinXiaohe;
    pinyin::init(flags);

    BOOST_CHECK(pinyin::match_pinyin(U'拼', U"pīnyīn", pinyin::PinyinFlag::Pinyin) == 3);
    BOOST_CHECK(pinyin::match_pinyin(U'拼', U"pinyin", pinyin::PinyinFlag::PinyinAscii) == 3);
    BOOST_CHECK(pinyin::match_pinyin(U'拼', U"pin1yin1", pinyin::PinyinFlag::PinyinAsciiDigit) == 4);
    BOOST_CHECK(pinyin::match_pinyin(U'拼', U"pbyb", pinyin::PinyinFlag::DoublePinyinXiaohe) == 2);
    BOOST_CHECK(pinyin::match_pinyin(U'拼', U"py", pinyin::PinyinFlag::InitialLetter) == 1);
    
    BOOST_CHECK(pinyin::match_pinyin(U'拼', U"pīnyīn", flags) == 3);
    BOOST_CHECK(pinyin::match_pinyin(U'拼', U"pinyin", flags) == 3);
    BOOST_CHECK(pinyin::match_pinyin(U'拼', U"pin1yin1", flags) == 4);
    BOOST_CHECK(pinyin::match_pinyin(U'拼', U"pbyb", flags) == 2);
    BOOST_CHECK(pinyin::match_pinyin(U'拼', U"py", flags) == 1);

    pinyin::destroy();
}

BOOST_AUTO_TEST_CASE(ReadChar32) {
    int len;
    BOOST_CHECK(pinyin::read_char32(U"\0", &len) == U'\0' && len == 1);
    BOOST_CHECK(pinyin::read_char32(U"a", &len) == U'a' && len == 1);
    BOOST_CHECK(pinyin::read_char32(U"¢", &len) == U'¢' && len == 2);
    BOOST_CHECK(pinyin::read_char32(U"拼", &len) == U'拼' && len == 3);
    BOOST_CHECK(pinyin::read_char32(U"𐍈", &len) == U'𐍈' && len == 4);
}