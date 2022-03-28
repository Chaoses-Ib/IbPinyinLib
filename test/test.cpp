#define BOOST_TEST_MODULE test
#include <boost/test/unit_test.hpp>

#define IB_PINYIN_ENCODING 32
#include <IbPinyin/pinyin.hpp>

#define LITERAL(s) IB_PINYIN_LITERAL(s)
#define LEN_8_16_32(a, b, c) (IB_PINYIN_ENCODING == 8 ? a : IB_PINYIN_ENCODING == 16 ? b : c)

BOOST_AUTO_TEST_CASE(MatchPinyin) {
    pinyin::init(pinyin::PinyinFlag::All);

    BOOST_CHECK(pinyin::match_pinyin(U'拼', LITERAL("pīnyīn"), pinyin::PinyinFlag::Pinyin) == 3);
    BOOST_CHECK(pinyin::match_pinyin(U'拼', LITERAL("pinyin"), pinyin::PinyinFlag::PinyinAscii) == 3);
    BOOST_CHECK(pinyin::match_pinyin(U'拼', LITERAL("pin1yin1"), pinyin::PinyinFlag::PinyinAsciiDigit) == 4);
    BOOST_CHECK(pinyin::match_pinyin(U'拼', LITERAL("py"), pinyin::PinyinFlag::InitialLetter) == 1);

    BOOST_CHECK(pinyin::match_pinyin(U'拼', LITERAL("pny;"), pinyin::PinyinFlag::DoublePinyinMicrosoft) == 2);
    BOOST_CHECK(pinyin::match_pinyin(U'英', LITERAL("y;"), pinyin::PinyinFlag::DoublePinyinMicrosoft) == 2);
    BOOST_CHECK(pinyin::match_pinyin(U'拼', LITERAL("pbyb"), pinyin::PinyinFlag::DoublePinyinXiaohe) == 2);
    
    pinyin::PinyinFlagValue flags = pinyin::PinyinFlag::Pinyin
        | pinyin::PinyinFlag::PinyinAsciiDigit | pinyin::PinyinFlag::PinyinAscii | pinyin::PinyinFlag::InitialLetter
        | pinyin::PinyinFlag::DoublePinyinXiaohe;
    BOOST_CHECK(pinyin::match_pinyin(U'拼', LITERAL("pīnyīn"), flags) == 3);
    BOOST_CHECK(pinyin::match_pinyin(U'拼', LITERAL("pinyin"), flags) == 3);
    BOOST_CHECK(pinyin::match_pinyin(U'拼', LITERAL("pin1yin1"), flags) == 4);
    BOOST_CHECK(pinyin::match_pinyin(U'拼', LITERAL("py"), flags) == 1);
    BOOST_CHECK(pinyin::match_pinyin(U'拼', LITERAL("pbyb"), flags) == 2);

    pinyin::destroy();
}

BOOST_AUTO_TEST_CASE(ReadChar32) {
    int len;
    BOOST_CHECK(pinyin::read_char32(LITERAL("\0"), &len) == U'\0' && len == LEN_8_16_32(1, 1, 1));
    BOOST_CHECK(pinyin::read_char32(LITERAL("a"), &len) == U'a' && len == LEN_8_16_32(1, 1, 1));
    BOOST_CHECK(pinyin::read_char32(LITERAL("¢"), &len) == U'¢' && len == LEN_8_16_32(2, 1, 1));
    BOOST_CHECK(pinyin::read_char32(LITERAL("拼"), &len) == U'拼' && len == LEN_8_16_32(3, 1, 1));
    BOOST_CHECK(pinyin::read_char32(LITERAL("𐍈"), &len) == U'𐍈' && len == LEN_8_16_32(4, 2, 1));
}