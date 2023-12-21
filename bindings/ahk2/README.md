# IbPinyinLib.AutoHotkey2
Binding for [AutoHotkey](https://www.autohotkey.com/) v2.

## Usage
```ahk
; Use IbPinyin32 for 32-bit AutoHotkey
; #Include <IbPinyin32>
#Include <IbPinyin>

IsMatch := IbPinyin_IsMatch("pysousuoeve", "拼音搜索Everything", IbPinyin_AsciiFirstLetter | IbPinyin_Ascii)

是否匹配 := 拼音_匹配("pysousuoeve", "拼音搜索Everything")

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
```

32 位相比 64 位的 DLL 体积小 0.3 MiB（1.5 → 1.2 MiB），内存占用少 0.2 MiB（2.16 → 1.93 MiB）。

## Build
```pwsh
.\bindings\ahk2\build.ps1
```
`IbPinyin32.ahk` will be generated.