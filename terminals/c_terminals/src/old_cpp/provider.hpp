// Copyright (C) 2020 luna_koly
//
// A level of indirection that 'just returns
// a terminal' selecting the available backend
// automatically.


#pragma once


#include "vt100.hpp"


namespace TerminalProvider {
    /**
     * Returns some terminal.
     */
    VT100Terminal & provide();
}