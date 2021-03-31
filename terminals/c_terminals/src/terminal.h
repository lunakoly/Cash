#include <stdbool.h>
#include <stddef.h>

#pragma once

#include "helpers.h"

struct Terminal {
    void * features;
    char * error;
    bool eof_found;
    /**
     * Returns false if an error occured
     * somewhere.
     */
    bool (*is_ok)(struct Terminal * self);
    /**
     * Returns the number of the columns and
     * the rows.
     * Returned values are of range [0, count - 1].
     */
    struct PairIntInt (*get_size)(struct Terminal * self);
    /**
     * Returns the width of the terminal window.
     */
    int (*get_columns)(struct Terminal * self);
    /**
     * Returns the cursor position.
     * Values are of range [0, count - 1].
     */
    struct PairIntInt (*get_cursor)(struct Terminal * self);
    /**
     * Sets the cursor position.
     * Values are of range [0, count - 1].
     */
    void (*set_cursor)(struct Terminal * self, struct PairIntInt position);
    /**
     * Prints the character to the screen and
     * moves the cursor forward by 1. This particular
     * movement step is usually fast.
     */
    void (*put)(struct Terminal * self, struct Char4 it);
    /**
     * Moves the cursor to the left
     * or at the end of the previous line.
     */
    void (*move_left)(struct Terminal * self);
    /**
     * Moves the cursor to the right
     * or at the beginning of the next line.
     */
    void (*move_right)(struct Terminal * self);
    /**
     * Moves the cursor down by the specified
     * number of lines.
     */
    void (*move_down)(struct Terminal * self, int count);
    /**
     * Moves the cursor up by the specified
     * number of lines.
     */
    void (*move_up)(struct Terminal * self, int count);
    /**
     * Moves the cursor to the specified
     * position on the same line.
     * Position must be in range [0, width - 1].
     */
    void (*move_directly)(struct Terminal * self, int position);
    /**
     * Shows the cursor.
     */
    void (*show_cursor)(struct Terminal * self);
    /**
     * Hides the cursor.
     */
    void (*hide_cursor)(struct Terminal * self);
    /**
     * Prompts the user to enter a line.
     * The line is returned when the user hits
     * Enter.
     */
    char * (*read_line)(struct Terminal * self);

    bool (*to_raw_mode)(struct Terminal * self);
    bool (*to_normal_mode)(struct Terminal * self);
};

bool default_is_ok(struct Terminal * self);

bool is_1_byte_utf8(const char * bytes);
bool is_2_byte_utf8(const char * bytes);
bool is_3_byte_utf8(const char * bytes);
bool is_4_byte_utf8(const char * bytes);

bool is_valid_utf8(const char * text);

char terminal_get_1_byte();

struct Char4 terminal_get();
