// Copyright (C) 2021 luna_koly

// for the common types
#include <stddef.h>

#include "helpers.h"

struct Char4 char4_new(const char * value) {
    struct Char4 result = {
        .values = "\0\0\0\0"
    };

    size_t it = 0;

    while (it < 4 && value[it] != '\0') {
        result.values[it] = value[it];
        it += 1;
    }

    return result;
}
