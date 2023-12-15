#ifndef ib_pinyin_H
#define ib_pinyin_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ib_pinyin ib_pinyin;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

bool ib_pinyin_is_match_u8(const char* pattern_data, size_t pattern_len, const char* haystack_data, size_t haystack_len, uint32_t pinyin_notations);

bool ib_pinyin_is_match_u8c(const uint8_t* pattern, const uint8_t* haystack, uint32_t pinyin_notations);
void ib_pinyin_destroy(ib_pinyin* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
