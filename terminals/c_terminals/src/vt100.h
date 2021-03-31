// Copyright (C) 2021 luna_koly
//
// See Microsoft docs for _getch() and ENABLE_VIRTUAL_TERMINAL_PROCESSING
// for more information about conio library and VT100 support.
// https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences
// https://docs.microsoft.com/en-us/cpp/c-runtime-library/reference/getch-getwch?view=vs-2019

#pragma once

#include "terminal.h"
#include "helpers.h"

struct PairIntInt vt100_get_size(struct Terminal * self);

int vt100_get_columns(struct Terminal * self);

struct PairIntInt vt100_get_cursor(struct Terminal * self);

void vt100_set_cursor(struct Terminal * self, struct PairIntInt position);

void vt100_put(struct Terminal * self, struct Char4 it);

void vt100_move_left(struct Terminal * self);

void vt100_move_right(struct Terminal * self);

void vt100_move_down(struct Terminal * self, int count);

void vt100_move_up(struct Terminal * self, int count);

void vt100_move_directly(struct Terminal * self, int position);

void vt100_show_cursor(struct Terminal * self);

void vt100_hide_cursor(struct Terminal * self);

char * vt100_read_line(struct Terminal * self);

bool vt100_to_raw_mode(struct Terminal * self);

bool vt100_to_normal_mode(struct Terminal * self);

struct Terminal create_vt100_terminal();
