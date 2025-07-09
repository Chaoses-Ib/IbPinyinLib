cargo build --package ib-pinyin-c -r
if (!$?) {
    throw "Build ib-pinyin-c failed"
}

$64DllSrc = Join-Path -Path $PSScriptRoot -ChildPath ../../target/release/ib_pinyin_c.dll
$64DllDst = Join-Path -Path $PSScriptRoot -ChildPath Lib/IbPinyin64.dll
Copy-Item -Path $64DllSrc -Destination $64DllDst -Force

# 32-bit
# rustup target add i686-pc-windows-msvc
cargo build --package ib-pinyin-c -r --target=i686-pc-windows-msvc
if (!$?) {
    throw "Build 32-bit ib-pinyin-c failed"
}

$32DllSrc = Join-Path -Path $PSScriptRoot -ChildPath ../../target/i686-pc-windows-msvc/release/ib_pinyin_c.dll
$32DllDst = Join-Path -Path $PSScriptRoot -ChildPath Lib/IbPinyin32.dll
Copy-Item -Path $32DllSrc -Destination $32DllDst -Force

$64LibSrc = Join-Path -Path $PSScriptRoot -ChildPath Lib/IbPinyin.ahk
$32LibDst = Join-Path -Path $PSScriptRoot -ChildPath Lib/IbPinyin32.ahk
(Get-Content $64LibSrc).Replace('IbPinyin64', 'IbPinyin32').Replace('v2.0 64-bit', 'v2.0 32-bit') | Set-Content $32LibDst