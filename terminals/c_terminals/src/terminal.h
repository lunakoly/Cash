// Copyright (C) 2021 luna_koly
//
// Defines the main terminal abstraction
// as well as some common terminal-related
// functions

// for the bool
#include <stdbool.h>
// for the common types
#include <stddef.h>

#pragma once

#include "helpers.h"

/**
 * Common functionality for any
 * terminal.
 */
struct Terminal {
    /**
     * Platform-specific backend.
     */
    void * features;
    /**
     * Human-readable message in case
     * something went wrong.
     */
    char * error;
    /**
     * Returns false if an error occured
     * somewhere.
     */
    bool (*is_ok)(struct Terminal * self);
    /**
     * Puts the terminal into the raw mode.
     * https://en.wikipedia.org/wiki/Seventh_Edition_Unix_terminal_interface#Input_modes
     */
    bool (*to_raw_mode)(struct Terminal * self);
    /**
     * Puts the terminal into the mode it used to be
     * before going to the raw mode.
     */
    bool (*to_normal_mode)(struct Terminal * self);
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
};

/**
 * The one and only implementation
 * for the `is_ok()`.
 */
bool default_is_ok(struct Terminal * self);

// This code has been taken from:
// https://stackoverflow.com/questions/1031645/how-to-detect-utf-8-in-plain-c

bool is_1_byte_utf8(const char * bytes);
bool is_2_byte_utf8(const char * bytes);
bool is_3_byte_utf8(const char * bytes);
bool is_4_byte_utf8(const char * bytes);

bool is_valid_utf8(const char * text);

/**
 * Returns a single byte from the
 * stdin without waiting (a crossplatform
 * `getch()`).
 */
char terminal_get_1_byte();

/**
 * Returns a single UTF-8 dynamic-width
 * character from the stdin.
 */
struct Char4 terminal_get();
