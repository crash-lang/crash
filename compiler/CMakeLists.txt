cmake_minimum_required(VERSION 3.20.0)

project(crashc)

find_package(LLVM REQUIRED CONFIG)

add_definitions(${LLVM_DEFINITIONS})
include_directories(${LLVM_INCLUDE_DIRS})

#target_link_libraries(crashc LLVM) No targets configured yet