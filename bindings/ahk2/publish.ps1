$dllSrc = Join-Path -Path $PSScriptRoot -ChildPath ../../target/release/ib_pinyin_c.dll
$dllDst = Join-Path -Path $PSScriptRoot -ChildPath Lib/IbPinyin.dll
Copy-Item -Path $dllSrc -Destination $dllDst -Force

$publishDir = Join-Path -Path $PSScriptRoot -ChildPath ../../target/publish
$publishZip = Join-Path -Path $publishDir -ChildPath IbPinyinLib.AHK2.zip
Compress-Archive -Path (Join-Path -Path $PSScriptRoot -ChildPath Lib) -DestinationPath $publishZip -Force