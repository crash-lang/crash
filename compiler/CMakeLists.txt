cmake_minimum_required(VERSION 3.20.0)

project(crashc)

find_package(LLVM CONFIG)

if(NOT LLVM_FOUND)
    message("LLVM Missing. Please execute the setup.sh script to install all dependencies.")
    return()
endif ()

add_definitions(${LLVM_DEFINITIONS})
include_directories(${LLVM_INCLUDE_DIRS})

add_subdirectory(crashc-ast)
add_subdirectory(crashc-lex)
add_subdirectory(crashc-parser)

#target_link_libraries(TARGET LLVM)