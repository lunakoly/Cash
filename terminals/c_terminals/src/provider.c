#include <stdlib.h>

#include "provider.h"

static struct Terminal current_terminal = {
    .features = NULL,
    .error = NULL,
    .eof_found = 0,
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

static bool is_terminal_initialized() {
    return current_terminal.features != NULL;
}

bool terminal_has_next() {
    if (!is_terminal_initialized()) {
        current_terminal = create_vt100_terminal();
    }

    return is_terminal_initialized() && !current_terminal.eof_found;
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
