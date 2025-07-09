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

i = 1
patterns = ''
map = ''
for romaji, kana_list in dic.items():
    print(romaji, kana_list)
    for kana in kana_list:
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