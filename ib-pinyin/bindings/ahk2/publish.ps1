& (Join-Path -Path $PSScriptRoot -ChildPath build.ps1)

$publishDir = Join-Path -Path $PSScriptRoot -ChildPath ../../target/publish
$publishZip = Join-Path -Path $publishDir -ChildPath IbPinyinLib.AHK2.zip
Compress-Archive -Path (Join-Path -Path $PSScriptRoot -ChildPath Lib) -DestinationPath $publishZip -Force