// Copyright (C) 2021 luna_koly
//
// Contains platform-independent implementations
// only.

// for the common types
#include <stdint.h>
// for debugging :)
#include <stdio.h>
// for malloc()
#include <stdlib.h>

#include "vt100.h"
#include "keys.h"

int vt100_get_columns(struct Terminal * self) {
    return self->get_size(self).first;
}

void vt100_put(struct Terminal * self, struct Char4 symbol) {
    if (is_1_byte_utf8(symbol.values)) {
        printf("%c", symbol.values[0]);
    } else if (is_2_byte_utf8(symbol.values)) {
        printf("%c%c", symbol.values[0], symbol.values[1]);
    } else if (is_3_byte_utf8(symbol.values)) {
        printf("%c%c%c", symbol.values[0], symbol.values[1], symbol.values[2]);
    } else if (is_4_byte_utf8(symbol.values)) {
        printf("%c%c%c%c", symbol.values[0], symbol.values[1], symbol.values[2], symbol.values[3]);
    }
}

void vt100_move_left(struct Terminal * self) {
    if (self->get_cursor(self).first != 0) {
        printf("\033D");
    } else {
        printf("\033A\033[%dG", self->get_columns(self));
    }
}

void vt100_move_right(struct Terminal * self) {
    if (self->get_cursor(self).first != self->get_columns(self) - 1) {
        printf("\033C");
    } else {
        printf("\033B\033[0G");
    }
}

void vt100_move_down(struct Terminal * self, int count) {
    printf("\033[%dB", count);
}

void vt100_move_up(struct Terminal * self, int count) {
    printf("\033[%dA", count);
}

void vt100_move_directly(struct Terminal * self, int position) {
    printf("\033[%dG", position + 1);
}

void vt100_show_cursor(struct Terminal * self) {
    printf("\033[?25h");
}

void vt100_hide_cursor(struct Terminal * self) {
    printf("\033[?25l");
}

/**
 * Quick alternative for
 * std::vector...
 */
struct Line {
    struct Char4 * contents;
    size_t capacity;
    size_t size;
};

/**
 * Returns a new Line with
 * the default capacity of 64.
 */
struct Line line_new() {
    return (struct Line) {
        .contents = malloc(sizeof(struct Char4) * 64),
        .capacity = 64,
        .size = 0
    };
}

/**
 * Inserts `it` the the end
 * of the line.
 */
void line_emplace_back(struct Char4 it, struct Line * line) {
    if (line->size >= line->capacity) {
        if (line->capacity * 2 < line->capacity) {
            printf("Error > Emm, perhaps the line is reeaaally long... > line->capacity * 2");
            return;
        }

        line->contents = realloc(line->contents, sizeof(struct Char4) * line->capacity * 2);
        line->capacity *= 2;

        if (line->contents == NULL) {
            printf("Error > Couldn't reallocate the line");
            return;
        }
    }

    line->contents[line->size] = it;
    line->size += 1;
}

/**
 * Inserts `it` at the specified position
 * within the line.
 */
void line_insert(struct Char4 it, size_t position, struct Line * line) {
    if (line->size >= line->capacity) {
        if (line->capacity * 2 < line->capacity) {
            printf("Error > Emm, perhaps the line is reeaaally long... > line->capacity * 2");
            return;
        }

        line->contents = realloc(line->contents, sizeof(struct Char4) * line->capacity * 2);
        line->capacity *= 2;

        if (line->contents == NULL) {
            printf("Error > Couldn't reallocate the line");
            return;
        }
    }

    for (size_t that = position; that < line->size; that++) {
        line->contents[that + 1] = line->contents[that];
    }

    line->contents[position] = it;
    line->size += 1;
}

/**
 * Removes a character at the specified
 * position from the line.
 */
void line_erase(size_t position, struct Line * line) {
    for (size_t that = position + 1; that < line->size; that++) {
        line->contents[that - 1] = line->contents[that];
    }

    line->contents[line->size] = char4_new("");
    line->size -= 1;
}

/**
 * Holds contextual information.
 */
struct Session {
    /**
     * The current ('virtual') line.
     */
    struct Line line;
    /**
     * Just a reference to the terminal.
     * Simplifies passing it around.
     */
    struct Terminal * terminal;
    /**
     * The caret position within
     * the line.
     */
    size_t position;
};

/**
 * Returns the resulting user input
 * as a single string of chars (UTF-8).
 */
char * session_compose(struct Session * session) {
    size_t length = 0;
    struct Char4 * next = session->line.contents;

    while (next < session->line.contents + session->line.size) {
        char * symbol = next->values;

        if (is_1_byte_utf8(symbol)) {
            length += 1;
        } else if (is_2_byte_utf8(symbol)) {
            length += 2;
        } else if (is_3_byte_utf8(symbol)) {
            length += 3;
        } else if (is_4_byte_utf8(symbol)) {
            length += 4;
        }

        next += 1;
    }

    char * result = malloc(length + 2);
    char * result_next = result;
    next = session->line.contents;

    while (next < session->line.contents + session->line.size) {
        char * symbol = next->values;

        if (is_1_byte_utf8(symbol)) {
            result_next[0] = symbol[0];
            result_next += 1;
        } else if (is_2_byte_utf8(symbol)) {
            result_next[0] = symbol[0];
            result_next[1] = symbol[1];
            result_next += 2;
        } else if (is_3_byte_utf8(symbol)) {
            result_next[0] = symbol[0];
            result_next[1] = symbol[1];
            result_next[2] = symbol[2];
            result_next += 3;
        } else if (is_4_byte_utf8(symbol)) {
            result_next[0] = symbol[0];
            result_next[1] = symbol[1];
            result_next[2] = symbol[2];
            result_next[3] = symbol[3];
            result_next += 4;
        }

        next += 1;
    }

    result_next[0] = '\n';
    result_next[1] = '\0';
    return result;
}


/**
 * Analyzes the next 'normal' character the user enters.
 */
void process_character_insert(struct Char4 it, struct Session * session) {
    struct Terminal * terminal = session->terminal;
    struct PairIntInt position = (terminal->get_cursor)(terminal);
    int width = (terminal->get_columns)(terminal);

    // how many characters left in the 'virtual' line
    size_t tail = session->line.size - session->position;
    // how many characters left in the 'real' line the cursor is at
    size_t first_line_end = width - position.first;

    if (session->position >= session->line.size) {
        line_emplace_back(it, &session->line);
    } else {
        line_insert(it, session->position, &session->line);
    }

    (terminal->put)(terminal, session->line.contents[session->position]);
    (terminal->hide_cursor)(terminal); // prevents blinking all over the place

    // forces the cursor to move to the next line
    if (width - position.first == 1) {
        (terminal->put)(terminal, char4_new(" "));
        (terminal->move_left)(terminal);
    }

    // we're at the right place, so remember that
    position = (terminal->get_cursor)(terminal);

    // reprint the line contents
    for (size_t index = session->position + 1; index < session->line.size; index++) {
        (terminal->put)(terminal, session->line.contents[index]);
    }

    // return back to the right place
    (terminal->set_cursor)(terminal, position);
    (terminal->show_cursor)(terminal);
    session->position += 1;
}


/**
 * Analyzes the next backspace character.
 */
void process_character_backspace(struct Session * session) {
    struct Terminal * terminal = session->terminal;

    (terminal->hide_cursor)(terminal); // prevents blinking all over the place
    (terminal->move_left)(terminal);
    session->position -= 1;

    // we're at the right place, so remember that
    struct PairIntInt position = (terminal->get_cursor)(terminal);

    line_erase(session->position, &session->line);

    // reprint the line contents
    for (size_t index = session->position; index < session->line.size; index++) {
        (terminal->put)(terminal, session->line.contents[index]);
    }

    // remove last character since
    // everything is moved to the left by 1
    (terminal->put)(terminal, char4_new(" "));
    // return back to the right place
    (terminal->set_cursor)(terminal, position);
    (terminal->show_cursor)(terminal);
}


/**
 * Analyzes the next character.
 */
void process_character(struct Char4 it, struct Session * session) {
    struct Terminal * terminal = session->terminal;

    if (it.values[0] > 0) {
        process_character_insert(it, session);
    }

    else if (it.values[0] == KEY_BACKSPACE && session->position > 0) {
        process_character_backspace(session);
    }

    else if (it.values[0] == KEY_RIGHT && session->line.size > 0 && session->position < session->line.size) {
        (terminal->move_right)(terminal);
        session->position += 1;
    }

    else if (it.values[0] == KEY_LEFT && session->position > 0) {
        (terminal->move_left)(terminal);
        session->position -= 1;
    }

    // there's nothing to implement, really;
    // all this code will only be used to
    // read the user command input;
    // as soon as the user launches an app
    // it'll be the terminal driver who will
    // handle the basic line editing and other
    // interaction

    else if (it.values[0] == KEY_SIGINT) {
        // printf("[This is not implemented yet :(]");
    }

    else if (it.values[0] == KEY_SIGSTOP) {
        // printf("[This is not implemented yet :(]");
    }

    else if (it.values[0] == KEY_EOF) {
        // printf("[This is not implemented yet :(]");
    }
}

char * vt100_read_line(struct Terminal * self) {
    (self->to_raw_mode)(self);

    struct Session session = {
        .line = line_new(),
        .terminal = self,
        .position = 0
    };

    struct Char4 it = terminal_get();

    while (it.values[0] != KEY_RETURN) {
        process_character(it, &session);
        it = terminal_get();
    }

    int columns = (self->get_columns)(self);
    (self->move_directly)(self, columns - 1);
    (self->put)(self, char4_new("\n"));

    (self->to_normal_mode)(self);
    return session_compose(&session);
}
