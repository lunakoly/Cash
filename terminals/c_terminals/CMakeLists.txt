cmake_minimum_required(VERSION 3.13)

if (WIN32)
    set(PLATFORM_SPECIFIC_FILES windows)
endif (WIN32)

if (UNIX)
    set(PLATFORM_SPECIFIC_FILES unix)
endif (UNIX)

project(
    Terminals
        VERSION 1.1
        DESCRIPTION "The interpreter I've always wanted?"
        LANGUAGES C
)

set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED True)

configure_file(
    "src/about.h.in"
    "src/about.h"
)

add_library(
    c_terminals
        STATIC
            src/keys.h
            src/helpers.h
            src/helpers.c
            src/terminal.h
            src/terminal.c
            src/vt100.h
            src/vt100.c
            src/provider.h
            src/provider.c
            "src/${PLATFORM_SPECIFIC_FILES}/helpers.h"
            "src/${PLATFORM_SPECIFIC_FILES}/terminal.c"
            "src/${PLATFORM_SPECIFIC_FILES}/vt100.c"
)

install(TARGETS c_terminals DESTINATION .)
