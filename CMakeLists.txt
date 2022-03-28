cmake_minimum_required(VERSION 3.2)

project(IbPinyin)

add_library(IbPinyin
    source/pinyin.cpp
    source/double_pinyin.cpp
    source/data.cpp
)
target_include_directories(IbPinyin PUBLIC include)
target_compile_features(IbPinyin PUBLIC cxx_std_17)
target_compile_definitions(IbPinyin PRIVATE IB_PINYIN_ENCODING=32)

# test
if(CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME)
    option(BUILD_TESTING "Build the testing tree." OFF)
    include(CTest)
    if(BUILD_TESTING)
        find_package(Boost REQUIRED COMPONENTS unit_test_framework)
        add_executable(IbPinyin_test test/test.cpp)
        target_link_libraries(IbPinyin_test
            PRIVATE IbPinyin
            PRIVATE Boost::unit_test_framework
        )
    endif()
endif()