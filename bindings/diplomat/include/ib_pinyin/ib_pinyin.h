#ifndef ib_pinyin_H
#define ib_pinyin_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "ib_pinyin.d.h"






bool ib_pinyin_is_match_u8(DiplomatStringView pattern, DiplomatStringView haystack, uint32_t pinyin_notations);

bool ib_pinyin_is_match_u16(DiplomatU16View pattern, size_t pattern_len, DiplomatU16View haystack, size_t haystack_len, uint32_t pinyin_notations);

bool ib_pinyin_is_match_u32(DiplomatU32View pattern, DiplomatU32View haystack, uint32_t pinyin_notations);

uint64_t ib_pinyin_find_match_u8(DiplomatStringView pattern, DiplomatStringView haystack, uint32_t pinyin_notations);

uint64_t ib_pinyin_find_match_u16(DiplomatU16View pattern, DiplomatU16View haystack, uint32_t pinyin_notations);

uint64_t ib_pinyin_find_match_u32(DiplomatU32View pattern, DiplomatU32View haystack, uint32_t pinyin_notations);

void ib_pinyin_destroy(ib_pinyin* self);





#endif // ib_pinyin_H
