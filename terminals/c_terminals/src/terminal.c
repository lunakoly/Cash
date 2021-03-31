#include "terminal.h"

bool default_is_ok(struct Terminal * self) {
    return self->error != NULL && self->features != NULL;
}

// This code has been taken from:
// https://stackoverflow.com/questions/1031645/how-to-detect-utf-8-in-plain-c

bool is_1_byte_utf8(const char * bytes) {
    return
        // ASCII
        // use bytes[0] <= 0x7F to allow ASCII control characters
        bytes[0] == 0x09 ||
        bytes[0] == 0x0A ||
        bytes[0] == 0x0D ||
        bytes[0] <= 0x7F ||
        0x20 <= bytes[0] && bytes[0] <= 0x7E;
}

bool is_2_byte_utf8(const char * bytes) {
    return
        // non-overlong 2-byte
        0xC2 <= bytes[0] && bytes[0] <= 0xDF &&
        0x80 <= bytes[1] && bytes[1] <= 0xBF;
}

bool is_3_byte_utf8(const char * bytes) {
    return
        // excluding overlongs
        (
            bytes[0] == 0xE0 &&
            0xA0 <= bytes[1] && bytes[1] <= 0xBF &&
            0x80 <= bytes[2] && bytes[2] <= 0xBF
        ) ||
        // straight 3-byte
        (
            (0xE1 <= bytes[0] && bytes[0] <= 0xEC || bytes[0] == 0xEE || bytes[0] == 0xEF) &&
            (0x80 <= bytes[1] && bytes[1] <= 0xBF) &&
            (0x80 <= bytes[2] && bytes[2] <= 0xBF)
        ) ||
        // excluding surrogates
        (
            bytes[0] == 0xED &&
            0x80 <= bytes[1] && bytes[1] <= 0x9F &&
            0x80 <= bytes[2] && bytes[2] <= 0xBF
        );
}

bool is_4_byte_utf8(const char * bytes) {
    return
        // planes 1-3
        (
            bytes[0] == 0xF0 &&
            0x90 <= bytes[1] && bytes[1] <= 0xBF &&
            0x80 <= bytes[2] && bytes[2] <= 0xBF &&
            0x80 <= bytes[3] && bytes[3] <= 0xBF
        ) ||
        // planes 4-15
        (
            0xF1 <= bytes[0] && bytes[0] <= 0xF3 &&
            0x80 <= bytes[1] && bytes[1] <= 0xBF &&
            0x80 <= bytes[2] && bytes[2] <= 0xBF &&
            0x80 <= bytes[3] && bytes[3] <= 0xBF
        ) ||
        // plane 16
        (
            bytes[0] == 0xF4 &&
            0x80 <= bytes[1] && bytes[1] <= 0x8F &&
            0x80 <= bytes[2] && bytes[2] <= 0xBF &&
            0x80 <= bytes[3] && bytes[3] <= 0xBF
        );
}

bool is_valid_utf8(const char * text) {
    if (!text)
        return 0;

    const unsigned char * bytes = (const unsigned char *) text;

    while (*bytes) {
        if (is_1_byte_utf8(bytes)) {
            bytes += 1;
            continue;
        }

        // non-overlong 2-byte
        if (is_2_byte_utf8(bytes)) {
            bytes += 2;
            continue;
        }

        if (is_3_byte_utf8(bytes)) {
            bytes += 3;
            continue;
        }

        if (is_4_byte_utf8(bytes)) {
            bytes += 4;
            continue;
        }

        return 0;
    }

    return 1;
}

struct Char4 terminal_get() {
    struct Char4 result = char4_new("");

    result.values[0] = terminal_get_1_byte();
    // printf("[got[0] ::: %d]\n", result.values[0]);

    if (result.values[0] <= 0 || is_1_byte_utf8(result.values)) {
        return result;
    }

    result.values[1] = terminal_get_1_byte();
    // printf("[got[1] ::: %d]\n", result.values[1]);

    if (is_2_byte_utf8(result.values)) {
        return result;
    }

    result.values[2] = terminal_get_1_byte();
    // printf("[got[2] ::: %d]\n", result.values[2]);

    if (is_3_byte_utf8(result.values)) {
        return result;
    }

    result.values[3] = terminal_get_1_byte();
    // printf("[got[3] ::: %d]\n", result.values[3]);
    return result;
}
