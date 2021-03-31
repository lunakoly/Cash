// Copyright (C) 2020 luna_koly
//
// Reads input from a VT100 terminal.


#pragma once


#include "vt100.hpp"


namespace VT100Reader {
    /**
     * Prompts the user to enter a line.
     * The line is returned when the user hits
     * Enter.
     */
    std::string read_line(VT100Terminal & terminal);
}
