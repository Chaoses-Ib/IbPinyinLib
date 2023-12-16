#Requires AutoHotkey v2.0

#Include <IbPinyin>

IsMatch := IbPinyin_IsMatch("pysousuoeve", "拼音搜索Everything", IbPinyin_AsciiFirstLetter | IbPinyin_Ascii)

是否匹配 := 拼音_匹配("pysousuoeve", "拼音搜索Everything")
MsgBox(是否匹配)

是否匹配 := 拼音_匹配("pysousuoeve", "拼音搜索Everything", 拼音_简拼 | 拼音_全拼)
MsgBox(是否匹配)