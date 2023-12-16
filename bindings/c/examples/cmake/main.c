#include <stdio.h>
#include <string.h>
#include <ib_pinyin/ib_pinyin.h>
#include <ib_pinyin/notation.h>

void test_u8() {
    const char *pattern = u8"pysousuoeve";
	const char *haystack = u8"拼音搜索Everything";
	// 0x3
	const PinyinNotation notations = PINYIN_NOTATION_ASCII_FIRST_LETTER | PINYIN_NOTATION_ASCII;

	printf("%d\n", ib_pinyin_is_match_u8(pattern, strlen(pattern), haystack, strlen(haystack), notations));

	printf("%d\n", ib_pinyin_is_match_u8c(pattern, haystack, notations));
}

void test_u16() {
    const char16_t *pattern = u"pysousuoeve";
	const char16_t *haystack = u"拼音搜索Everything";
	// 0x3
	const PinyinNotation notations = PINYIN_NOTATION_ASCII_FIRST_LETTER | PINYIN_NOTATION_ASCII;

	printf("%d\n", ib_pinyin_is_match_u16(
		pattern, 
		sizeof(u"pysousuoeve") / sizeof(char16_t) - 1,
		haystack, 
		sizeof(u"拼音搜索Everything") / sizeof(char16_t) - 1,
		notations
	));

	printf("%d\n", ib_pinyin_is_match_u16c(pattern, haystack, notations));
}

void test_u32() {
    const char32_t *pattern = U"pysousuoeve";
	const char32_t *haystack = U"拼音搜索Everything";
	// 0x3
	const PinyinNotation notations = PINYIN_NOTATION_ASCII_FIRST_LETTER | PINYIN_NOTATION_ASCII;

	printf("%d\n", ib_pinyin_is_match_u32(
		pattern, 
		sizeof(U"pysousuoeve") / sizeof(char32_t) - 1,
		haystack, 
		sizeof(U"拼音搜索Everything") / sizeof(char32_t) - 1,
		notations
	));

	printf("%d\n", ib_pinyin_is_match_u32c(pattern, haystack, notations));
}

int main()
{
	test_u8();
	test_u16();
	test_u32();

	return 0;
}
