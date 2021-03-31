// Copyright (C) 2020 luna_koly
//
// See Microsoft docs for _getch() and ENABLE_VIRTUAL_TERMINAL_PROCESSING
// for more information about conio library and VT100 support.
// https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences
// https://docs.microsoft.com/en-us/cpp/c-runtime-library/reference/getch-getwch?view=vs-2019


#include "features.hpp"

#include <iostream>
#include <sstream>
// for _getch()
#include <conio.h>
// for requesting info about the console
#include <windows.h>

#include "../keys.hpp"

#include "../../windows/helpers.hpp"


WindowsFeatures::WindowsFeatures() {
    // Set output mode to handle virtual terminal sequences
    hOut = GetStdHandle(STD_OUTPUT_HANDLE);

    if (hOut == INVALID_HANDLE_VALUE) {
        error = report_error("Terminal > Invalid file handle for the standard output");
        return;
    }

    if (!GetConsoleMode(hOut, &original_configuration)) {
        error = report_error("Terminal > Couldn't get current console mode");
        return;
    }

    raw_configuration = original_configuration;
    raw_configuration |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
}


WindowsFeatures::~WindowsFeatures() {
    to_normal_mode();
}


bool WindowsFeatures::to_raw_mode() {
    if (!SetConsoleMode(hOut, raw_configuration)) {
        error = report_error("Terminal > Couldn't set the raw console mode");
        return false;
    }

    return true;
}


bool WindowsFeatures::to_normal_mode() {
    if (!SetConsoleMode(hOut, original_configuration)) {
        error = report_error("Terminal > Couldn't recover from the raw console mode");
        return false;
    }

    return true;
}


int WindowsFeatures::get() {
    auto it = _getch();

    // this is a control character
    if (it == 0 || it == 224) {
        // the code describing the real key
        it = _getch();

        switch (it) {
            case RAW_KEY_UP:
                return KEY_UP;
            case RAW_KEY_DOWN:
                return KEY_DOWN;
            case RAW_KEY_RIGHT:
                return KEY_RIGHT;
            case RAW_KEY_LEFT:
                return KEY_LEFT;
            case 83:
                return KEY_DELETE;
            default:
                return it;
        }
    } else if (it == RAW_KEY_ESCAPE) {
        return KEY_ESCAPE;
    } else if (it == RAW_KEY_BACKSPACE) {
        return KEY_BACKSPACE;
    } else if (it == RAW_KEY_RETURN) {
        return KEY_RETURN;
    } else if (it == RAW_KEY_TAB) {
        return KEY_TAB;
    }

    return it;
}


std::pair<int, int> WindowsFeatures::get_size() {
    CONSOLE_SCREEN_BUFFER_INFO info;
    GetConsoleScreenBufferInfo(hOut, &info);

    auto columns = info.srWindow.Right  - info.srWindow.Left + 1;
    auto rows    = info.srWindow.Bottom - info.srWindow.Top  + 1;

    return std::make_pair(columns, rows);
}


int WindowsFeatures::get_columns() {
    return get_size().first;
}


std::pair<int, int> WindowsFeatures::get_cursor() {
    CONSOLE_SCREEN_BUFFER_INFO info;
    GetConsoleScreenBufferInfo(hOut, &info);
    return std::make_pair(info.dwCursorPosition.X, info.dwCursorPosition.Y);
}


void WindowsFeatures::set_cursor(std::pair<int, int> position) {
    auto columns = get_columns();

    if (columns > 0) {
        while (position.first >= columns) {
            position.first -= columns;
            position.second += 1;
        }
    }

    SetConsoleCursorPosition(hOut, COORD{(SHORT) position.first, (SHORT) position.second});
}
