cargo build --package ib-pinyin-c -r
if (!$?) {
    throw "Build ib-pinyin-c failed"
}

.\bindings\ahk2\publish.ps1