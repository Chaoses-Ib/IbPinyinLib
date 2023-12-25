#Requires AutoHotkey v2.0

; Use IbPinyin32 for 32-bit AutoHotkey
; #Include <IbPinyin32>
#Include <IbPinyin>

IsMatch := IbPinyin_IsMatch("pysousuoeve", "拼音搜索Everything", IbPinyin_AsciiFirstLetter | IbPinyin_Ascii)
MsgBox(IsMatch)

是否匹配 := 拼音_匹配("pysousuoeve", "拼音搜索Everything")
MsgBox(是否匹配)

; 拼音_简拼
; 拼音_全拼
; 拼音_带声调全拼
; 拼音_Unicode
; 拼音_智能ABC双拼
; 拼音_拼音加加双拼
; 拼音_微软双拼
; 拼音_华宇双拼
; 拼音_紫光双拼
; 拼音_小鹤双拼
; 拼音_自然码双拼
是否匹配 := 拼音_匹配("pysousuoeve", "拼音搜索Everything", 拼音_简拼 | 拼音_全拼)
MsgBox(是否匹配)