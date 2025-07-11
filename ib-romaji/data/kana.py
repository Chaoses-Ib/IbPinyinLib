# /// script
# requires-python = ">=3.9"
# dependencies = [
#     "requests",
# ]
# ///
import requests

dic: dict[str, list[str]] = {}
half = False
def f(txt):
    global half
    for line in txt.splitlines():
        if line.startswith(';;'):
            if line == ';; half kana mappings':
                half = True
            continue
        # print(line.split(' '))
        romaji, kana = line.split(' ')
        if kana.startswith('\\U'):
            kana = int(kana[2:], 16)
            kana = f'\\u{{{kana:x}}}'
        if dic.get(romaji) is None:
            dic[romaji] = [kana]
        else:
            dic[romaji].append(kana)
txt = requests.get('http://codeberg.org/miurahr/pykakasi/raw/commit/4f26c75fed807046ddf5187c8fa190467b36ee79/src/data/hepburndict.utf8').text
f(txt)
txt = requests.get('https://codeberg.org/miurahr/pykakasi/raw/commit/4f26c75fed807046ddf5187c8fa190467b36ee79/src/data/hepburnhira.utf8').text
f(txt)

kanas = {}
for romaji, kana_list in dic.items():
    kana_list_set = set(kana_list)
    if len(kana_list_set) != len(kana_list):
        print(f'Duplicate kana for {romaji}: {kana_list} -> {kana_list_set}')
    else:
        print(romaji, kana_list)
    for kana in kana_list_set:
        if kana in kanas:
            raise ValueError(f'Duplicate kana: {kana} for {romaji} and {kanas[kana]}')
        kanas[kana] = romaji
kanas = dict(sorted(kanas.items(), key=lambda item: chr(int(item[0].removeprefix('\\u{').removesuffix('}'), 16)) if item[0].startswith('\\u{') else item[0]))

i = 1
patterns = ''
map = ''
for kana, romaji in kanas.items():
    kana = kana.replace('"', '\\"')
    romaji = romaji.replace('"', '\\"')

    patterns += f'"{kana}",'
    map += f'"{romaji}",'
    
    if i % 8 == 0:
        patterns += '\n'
        map += '\n'
    i += 1
print('patterns:')
print(patterns)
print('map:')
print(map)