// Copyright (C) 2021 luna_koly
//
// A level of indirection that 'just returns
// a TerminalID' selecting the available backend
// automatically.

#pragma once

#include <stdint.h>

#include "terminal.h"
#include "vt100.h"

bool terminal_has_next();

char * terminal_read_line();

void deallocate_string(char * pointer);
