#include <stdio.h>
#include <string.h>
#include <ib_pinyin/ib_pinyin.h>
#include <ib_pinyin/notation.h>

int main()
{
	const char *pattern = u8"pysousuoeve";
	const char *haystack = u8"拼音搜索Everything";
	// 0x3
	const PinyinNotation notations = PINYIN_NOTATION_ASCII_FIRST_LETTER | PINYIN_NOTATION_ASCII;

	printf("%d\n", ib_pinyin_is_match_u8(pattern, strlen(pattern), haystack, strlen(haystack), notations));

	printf("%d\n", ib_pinyin_is_match_u8c(pattern, haystack, notations));

	return 0;
}
