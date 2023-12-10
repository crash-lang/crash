cmake_minimum_required(VERSION 3.17)
project(crash)

set(CMAKE_CXX_STANDARD 20)

add_subdirectory(lib/crash-lexer)
add_subdirectory(crash-compiler)