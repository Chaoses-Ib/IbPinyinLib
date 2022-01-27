pinyins = []
pinyin_combinations = []
pinyin_tables = []

def read_pinyins(f):
    while line := f.readline()[:-1]:
        lst = line.split(',')
        pinyins.append(tuple(lst[i] for i in (0,2,3)))

def read_pinyin_combinations(f):
    while line := f.readline()[:-1]:
        pinyin_combinations.append(line)

def read_pinyin_tables(f):
    while line := f.readline()[:-1]:
        rng = line[:-1].split(',')
        pinyin_tables.append((range(int(rng[0], 16), int(rng[1], 16) + 1), f.readline()[:-1]))

def output():
    print(f'extern Pinyin pinyins[{len(pinyins)}];')
    pinyin_code = f'Pinyin pinyins[{len(pinyins)}] {{\n'
    for pinyin in pinyins:
        '''
        s = ",".join(f'IB_PINYIN_LITERAL("{ py }")' for py in pinyin)
        pinyin_code += f'{{{ s }}},\n'
        '''
        pinyin_code += f'P({pinyin[0]})'
    pinyin_code += '};\n'

    max_comb = max(comb.count(",") for comb in pinyin_combinations) + 1  # 10
    print(f'extern PinyinCombination<{max_comb}> pinyin_combinations[{len(pinyin_combinations)}];')
    comb_code = f'PinyinCombination<{max_comb}> pinyin_combinations[{len(pinyin_combinations)}] {{\n'
    for comb in pinyin_combinations:
        comb_code += f'{{{ comb.count(",") + 1 },{{{ comb }}}}},'
    comb_code += '};\n'

    print(f'extern PinyinRange pinyin_ranges[{len(pinyin_tables)}];')
    table_code = ''
    range_code = f'PinyinRange pinyin_ranges[{len(pinyin_tables)}] {{\n'
    for table in pinyin_tables:
        table_name = f'pinyin_table_{ table[0].start :X}_{ table[0].stop - 1 :X}'
        table_code += f'uint16_t { table_name }[] = {{{ table[1].replace(",65535", ",F") }}};\n'
        range_code += f'{{0x{ table[0].start :X}, 0x{ table[0].stop - 1 :X}, { table_name }}},\n'
    range_code += '};'

    return f'''#include "pch.h"
#include "Pinyin.hpp"

#define P(s) {{IB_PINYIN_LITERAL(#s)}},
#define F 65535

namespace pinyin {{
{pinyin_code}
{comb_code}
{table_code}
{range_code}
}}
'''

with open('data/pinyin_compact.txt', encoding='utf8') as f:
    if f.readline() == 'pinyins:\n':
        read_pinyins(f)
    if f.readline() == 'pinyin_combinations:\n':
        read_pinyin_combinations(f)
    if f.readline() == 'pinyin_tables:\n':
        read_pinyin_tables(f)
    with open('data.cpp', 'w', encoding='utf-8-sig') as f:
        f.write(output())