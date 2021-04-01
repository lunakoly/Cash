// Copyright (C) 2021 luna_koly
//
// Platform-specific details.

// for debugging :)
#include <stdio.h>
// for requesting info about the console
#include <windows.h>

#include "../keys.h"
#include "../helpers.h"
#include "../terminal.h"
#include "../vt100.h"
#include "helpers.h"

struct VT100Terminal {
    /**
     * Used for calls to WINAPI.
     */
    HANDLE hOut;
    /**
     * Holds the original terminal configuration.
     * We must restore it when the app exits.
     */
    DWORD original_configuration;
    /**
     * The configuration to use when we need
     * to read user input.
     */
    DWORD raw_configuration;
};

struct Terminal create_vt100_terminal() {
    struct Terminal terminal = {
        .features = NULL,
        .error = NULL,
        .is_ok = default_is_ok,
        .get_size = vt100_get_size,
        .get_columns = vt100_get_columns,
        .get_cursor = vt100_get_cursor,
        .set_cursor = vt100_set_cursor,
        .put = vt100_put,
        .move_left = vt100_move_left,
        .move_right = vt100_move_right,
        .move_down = vt100_move_down,
        .move_up = vt100_move_up,
        .move_directly = vt100_move_directly,
        .show_cursor = vt100_show_cursor,
        .hide_cursor = vt100_hide_cursor,
        .read_line = vt100_read_line,
        .to_raw_mode = vt100_to_raw_mode,
        .to_normal_mode = vt100_to_normal_mode
    };

    // Set output mode to handle virtual terminal sequences
    HANDLE hOut = GetStdHandle(STD_OUTPUT_HANDLE);

    if (hOut == INVALID_HANDLE_VALUE) {
        terminal.error = report_error("Terminal > Invalid file handle for the standard output");
        return terminal;
    }

    DWORD original_configuration = 0;

    if (!GetConsoleMode(hOut, &original_configuration)) {
        terminal.error = report_error("Terminal > Couldn't get current console mode");
        return terminal;
    }

    DWORD raw_configuration = original_configuration;
    raw_configuration |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;

    struct VT100Terminal * features = malloc(sizeof(struct VT100Terminal));

    *features = (struct VT100Terminal) {
        .hOut = hOut,
        .original_configuration = original_configuration,
        .raw_configuration = raw_configuration,
    };

    terminal.features = features;
    return terminal;
}

#define FEATURES ((struct VT100Terminal *) self->features)

bool vt100_to_raw_mode(struct Terminal * self) {
    if (!SetConsoleMode(FEATURES->hOut, FEATURES->raw_configuration)) {
        self->error = report_error("Terminal > Couldn't set the raw console mode");
        return false;
    }

    return true;
}

bool vt100_to_normal_mode(struct Terminal * self) {
    if (!SetConsoleMode(FEATURES->hOut, FEATURES->original_configuration)) {
        self->error = report_error("Terminal > Couldn't recover from the raw console mode");
        return false;
    }

    return true;
}

struct PairIntInt vt100_get_size(struct Terminal * self) {
    CONSOLE_SCREEN_BUFFER_INFO info;
    GetConsoleScreenBufferInfo(FEATURES->hOut, &info);

    auto columns = info.srWindow.Right  - info.srWindow.Left + 1;
    auto rows    = info.srWindow.Bottom - info.srWindow.Top  + 1;

    return (struct PairIntInt) { columns, rows };
}

struct PairIntInt vt100_get_cursor(struct Terminal * self) {
    CONSOLE_SCREEN_BUFFER_INFO info;
    GetConsoleScreenBufferInfo(FEATURES->hOut, &info);
    return (struct PairIntInt) { info.dwCursorPosition.X, info.dwCursorPosition.Y };
}

void vt100_set_cursor(struct Terminal * self, struct PairIntInt position) {
    auto columns = self->get_columns(self);

    if (columns > 0) {
        while (position.first >= columns) {
            position.first -= columns;
            position.second += 1;
        }
    }

    SetConsoleCursorPosition(FEATURES->hOut, (COORD) { (SHORT) position.first, (SHORT) position.second });
}
