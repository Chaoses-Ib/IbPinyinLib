$dllSrc = Join-Path -Path $PSScriptRoot -ChildPath ../../target/release/ib_pinyin_c.dll
$dllDst = Join-Path -Path $PSScriptRoot -ChildPath Lib/IbPinyin.dll
Copy-Item -Path $dllSrc -Destination $dllDst -Force