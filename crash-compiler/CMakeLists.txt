set(SOURCE_FILES src/compiler.cpp)

add_executable(crash-compiler ${SOURCE_FILES})

target_link_libraries(crash-compiler LINK_PRIVATE crash-lexer)