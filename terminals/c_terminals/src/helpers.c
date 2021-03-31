#include <stddef.h>

#include "helpers.h"

struct Char4 char4_new(const char * value) {
    struct Char4 result = {
        .values = "\0\0\0\0"
    };

    for (size_t it = 0; it < 4; it++) {
        result.values[it] = value[it];
    }

    return result;
}
