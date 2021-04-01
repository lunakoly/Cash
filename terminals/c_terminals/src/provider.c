// Copyright (C) 2021 luna_koly

// for free()
#include <stdlib.h>
// for debugging :)
#include <stdio.h>

#include "provider.h"

/**
 * The current global terminal instance.
 */
static struct Terminal current_terminal = {
    .features = NULL,
    .error = NULL,
    .is_ok = NULL,
    .get_size = NULL,
    .get_columns = NULL,
    .get_cursor = NULL,
    .put = NULL,
    .move_left = NULL,
    .move_right = NULL,
    .move_down = NULL,
    .move_up = NULL,
    .move_directly = NULL,
    .show_cursor = NULL,
    .hide_cursor = NULL,
    .read_line = NULL,
    .to_raw_mode = NULL,
    .to_normal_mode = NULL
};

/**
 * Returns true if we managed
 * to acquire some sort of a
 * terminal.
 */
static bool is_terminal_initialized() {
    return current_terminal.features != NULL;
}

bool terminal_has_next() {
    if (!is_terminal_initialized()) {
        current_terminal = create_vt100_terminal();
    }

    return is_terminal_initialized() && !feof(stdin);
}

char * terminal_read_line() {
    if (!is_terminal_initialized()) {
        current_terminal = create_vt100_terminal();
    }

    return current_terminal.read_line(&current_terminal);
}

void deallocate_string(char * pointer) {
    free(pointer);
}
