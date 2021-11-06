#define BOOST_TEST_MODULE Test
#include <boost/test/unit_test.hpp>

#define IB_PINYIN_ENCODING 8
#include "../PinyinLib/Pinyin.hpp"

BOOST_AUTO_TEST_CASE(MatchPinyin) {
    BOOST_CHECK(pinyin::match_pinyin(u8"pīnyīn", U'拼', pinyin::PinyinFlag::Pinyin) == 3);
    BOOST_CHECK(pinyin::match_pinyin(u8"pinyin", U'拼', pinyin::PinyinFlag::PinyinAscii) == 3);
    BOOST_CHECK(pinyin::match_pinyin(u8"pin1yin1", U'拼', pinyin::PinyinFlag::PinyinAsciiDigit) == 4);
    BOOST_CHECK(pinyin::match_pinyin(u8"pbyb", U'拼', pinyin::PinyinFlag::DoublePinyinXiaohe) == 2);
    BOOST_CHECK(pinyin::match_pinyin(u8"py", U'拼', pinyin::PinyinFlag::Initial) == 1);

    pinyin::PinyinFlagValue flags = pinyin::PinyinFlag::Pinyin | pinyin::PinyinFlag::PinyinAscii | pinyin::PinyinFlag::PinyinAsciiDigit
        | pinyin::PinyinFlag::DoublePinyinXiaohe | pinyin::PinyinFlag::Initial;
    BOOST_CHECK(pinyin::match_pinyin(u8"pīnyīn", U'拼', flags) == 3);
    BOOST_CHECK(pinyin::match_pinyin(u8"pinyin", U'拼', flags) == 3);
    BOOST_CHECK(pinyin::match_pinyin(u8"pin1yin1", U'拼', flags) == 4);
    BOOST_CHECK(pinyin::match_pinyin(u8"pbyb", U'拼', flags) == 2);
    BOOST_CHECK(pinyin::match_pinyin(u8"py", U'拼', flags) == 1);
}

BOOST_AUTO_TEST_CASE(ReadChar32) {
    int len;
    BOOST_CHECK(pinyin::read_char32(u8"\0", &len) == U'\0' && len == 1);
    BOOST_CHECK(pinyin::read_char32(u8"a", &len) == U'a' && len == 1);
    BOOST_CHECK(pinyin::read_char32(u8"¢", &len) == U'¢' && len == 2);
    BOOST_CHECK(pinyin::read_char32(u8"拼", &len) == U'拼' && len == 3);
    BOOST_CHECK(pinyin::read_char32(u8"𐍈", &len) == U'𐍈' && len == 4);
}