// Copyright (C) 2021 luna_koly
//
// Some code that automatically
// instantiates the proper terminal
// and provides the useful functions
// themselves.

#pragma once

// for the common types
#include <stdint.h>

#include "terminal.h"
#include "vt100.h"

/**
 * Returns true if we can
 * ask the user for more input.
 * In case of EOF that would be
 * false, for example.
 */
bool terminal_has_next();

/**
 * Prompts a single line.
 * The last character is always `\n`.
 */
char * terminal_read_line();

/**
 * Used to deallocate the line returned by
 * `terminal_read_line`, when it's not
 * needed any more.
 */
void deallocate_string(char * pointer);

/**
 * Used to determine if the current
 * stdin is provided as an interactive
 * user session or a simple file redirect.
 */
bool terminal_is_interactive();
