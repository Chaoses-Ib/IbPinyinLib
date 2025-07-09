# CMakeList.txt : CMake project for cmake, include source and define
# project specific logic here.
#
cmake_minimum_required(VERSION 3.8)

project("cmake")

# Add source to this project's executable.
add_executable(cmake "main.c")

target_include_directories(cmake PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/../../include)

# target_link_libraries(cmake PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/../../../../target/debug/ib_pinyin_c.lib)
target_link_libraries(cmake PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/../../../../target/debug/ib_pinyin_c.dll.lib)
add_custom_command(TARGET cmake POST_BUILD
  COMMAND ${CMAKE_COMMAND} -E copy_if_different
  "${CMAKE_CURRENT_SOURCE_DIR}/../../../../target/debug/ib_pinyin_c.dll"
  $<TARGET_FILE_DIR:cmake>)
