# IbPinyinLib.C++
一个 C++ 拼音库。

* 支持 Unicode 辅助平面汉字
* 支持拼音格式：
    * 拼音
    * ASCII 拼音
    * 带声调 ASCII 拼音
    * 智能 ABC 双拼
    * 拼音加加双拼
    * 微软双拼
    * 华宇双拼（紫光双拼）
    * 小鹤双拼
    * 自然码双拼

## CMake
```cmake
cmake_minimum_required(VERSION 3.14)

include(FetchContent)
FetchContent_Declare(IbPinyin
    GIT_REPOSITORY https://github.com/Chaoses-Ib/IbPinyinLib.git
    GIT_TAG        e576e81ca06a297436bba7b124630b5d64e3106f
)
FetchContent_MakeAvailable(IbPinyin)
```