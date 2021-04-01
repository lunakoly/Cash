// Copyright (C) 2021 luna_koly
//
// Platform-specific details.
//
// See Microsoft docs for _getch() and ENABLE_VIRTUAL_TERMINAL_PROCESSING
// for more information about conio library and VT100 support.
// https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences
// https://docs.microsoft.com/en-us/cpp/c-runtime-library/reference/getch-getwch?view=vs-2019

// for _getch()
#include <conio.h>
// for debugging :)
#include <stdio.h>

#include "../keys.h"
#include "../terminal.h"

char terminal_get_1_byte() {
    int it = _getch();
    // printf("<getch ::: %d>\n", it);

    // this is a control character
    if (it == 0 || it == 224) {
        // the code describing the real key
        it = _getch();
        // printf("<getch ::: %d>\n", it);

        switch (it) {
            case RAW_KEY_UP:
                return KEY_UP;
            case RAW_KEY_DOWN:
                return KEY_DOWN;
            case RAW_KEY_RIGHT:
                return KEY_RIGHT;
            case RAW_KEY_LEFT:
                return KEY_LEFT;
            case 83:
                return KEY_DELETE;
            default:
                return it;
        }
    } else if (it == RAW_KEY_ESCAPE) {
        return KEY_ESCAPE;
    } else if (it == RAW_KEY_BACKSPACE) {
        return KEY_BACKSPACE;
    } else if (it == RAW_KEY_RETURN) {
        return KEY_RETURN;
    } else if (it == RAW_KEY_TAB) {
        return KEY_TAB;
    } else if (it == RAW_KEY_SIGINT) {
        return KEY_SIGINT;
    } else if (it == RAW_KEY_SIGSTOP) {
        return KEY_SIGSTOP;
    } else if (it == RAW_KEY_EOF) {
        return KEY_EOF;
    }

    return it;
}
