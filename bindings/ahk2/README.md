# IbPinyinLib.AutoHotkey2
Binding for [AutoHotkey](https://www.autohotkey.com/) v2.

## Usage
```ahk
#Include <IbPinyin>

IsMatch := IbPinyin_IsMatch("pysousuoeve", "拼音搜索Everything", IbPinyin_AsciiFirstLetter | IbPinyin_Ascii)

是否匹配 := 拼音_匹配("pysousuoeve", "拼音搜索Everything")
是否匹配 := 拼音_匹配("pysousuoeve", "拼音搜索Everything", 拼音_简拼 | 拼音_全拼)
```