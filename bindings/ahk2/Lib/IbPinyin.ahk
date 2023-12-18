; IbPinyinLib
; https://github.com/Chaoses-Ib/IbPinyinLib

#Requires AutoHotkey v2.0

#DllLoad IbPinyin.dll

IbPinyin_Unicode := 0x8
IbPinyin_Ascii := 0x2
IbPinyin_AsciiTone := 0x4
IbPinyin_AsciiFirstLetter := 0x1
IbPinyin_DiletterAbc := 0x10
IbPinyin_DiletterJiajia := 0x20
IbPinyin_DiletterMicrosoft := 0x40
IbPinyin_DiletterThunisoft := 0x80
IbPinyin_DiletterXiaohe := 0x100
IbPinyin_DiletterZrm := 0x200

IbPinyin_IsMatch(pattern, haystack, notations := IbPinyin_AsciiFirstLetter | IbPinyin_Ascii)
{
    return DllCall("IbPinyin\ib_pinyin_is_match_u16", "Ptr", StrPtr(pattern), "UPtr", StrLen(pattern), "Ptr", StrPtr(haystack), "UPtr", StrLen(haystack), "UInt", notations, "Cdecl Int") = 1
}

拼音_简拼 := 1
拼音_全拼 := 2
拼音_带声调全拼 := 4
拼音_Unicode := 8
拼音_智能ABC双拼 := 16
拼音_拼音加加双拼 := 32
拼音_微软双拼 := 64
拼音_华宇双拼 := 128
拼音_紫光双拼 := 128
拼音_小鹤双拼 := 256
拼音_自然码双拼 := 512

拼音_匹配(关键字, 文本, 拼音 := 拼音_简拼 | 拼音_全拼) {
    return IbPinyin_IsMatch(关键字, 文本, 拼音)
}