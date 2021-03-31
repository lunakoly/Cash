// Copyright (C) 2020 luna_koly
//
// See this guide for more info about implementing
// a command line input:
// https://viewsourcecode.org/snaptoken/kilo/02.enteringRawMode.html
//
// Use `sed -n l` to find out all possible codes
// for keys.


#include "features.hpp"

#include <iostream>
// for read(), STDIN_FILENO
#include <unistd.h>
// for tcgetattr(), tcsetattr(), termios, flags, TCSAFLUSH
#include <termios.h>
// for iscntrl()
#include <ctype.h>
// for requesting terminal size
#include <sys/ioctl.h>
// for scanf()
// #include <stdio.h>
// for std::strerror()
#include <cstring>
#include <climits>
#include <mutex>

#include "../keys.hpp"

#include "../../unix/helpers.hpp"


/**
 * Just stops everything.
 */
static void die(const char *s) {
    std::cout << "[DEATH]" << std::endl;
    perror(s);
    exit(1);
}


UnixFeatures::UnixFeatures() {
    if (tcgetattr(STDIN_FILENO, &original_configuration) == -1) {
        error = report_error("Terminal > Couldn't get current console mode");
        return;
    }

    // I left some random flags here
    // just in case
    raw_configuration = original_configuration;
    raw_configuration.c_lflag &= ~ECHO;
    raw_configuration.c_lflag &= ~ICANON;
    // raw_configuration.c_lflag &= ~IXON;
    // raw_configuration.c_lflag &= ~IEXTEN;
    // raw_configuration.c_lflag &= ~ICRNL;
}


UnixFeatures::~UnixFeatures() {
    to_normal_mode();
}


bool UnixFeatures::to_raw_mode() {
    if (tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw_configuration) == -1) {
        error = report_error("Terminal > Couldn't set the raw console mode");
        return false;
    }

    return true;
}


bool UnixFeatures::to_normal_mode() {
    if (tcsetattr(STDIN_FILENO, TCSAFLUSH, &original_configuration) == -1) {
        error = report_error("Terminal > Couldn't recover from the raw console mode");
        error = std::strerror(errno);
        return false;
    }

    return true;
}


int UnixFeatures::get() {
    char it = std::cin.get();

    // this is a control character
    if (it == RAW_KEY_ESCAPE) {
        // just `[`
        it = std::cin.get();

        if (it == '[') {
            // the code describing the real key
            it = std::cin.get();

            switch (it) {
                case RAW_KEY_UP:
                    return KEY_UP;
                case RAW_KEY_DOWN:
                    return KEY_DOWN;
                case RAW_KEY_RIGHT:
                    return KEY_RIGHT;
                case RAW_KEY_LEFT:
                    return KEY_LEFT;
                case '0'...'9':
                    it = std::cin.get();

                    if (it == '~') {
                        return KEY_DELETE;
                    }

                    return it;
                default:
                    return it;
            }
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


std::pair<int, int> UnixFeatures::get_size() {
    winsize size;
    ioctl(STDOUT_FILENO, TIOCGWINSZ, &size);
    return std::make_pair(size.ws_col, size.ws_row);
}


int UnixFeatures::get_columns() {
    return get_size().first;
}


std::pair<int, int> UnixFeatures::get_cursor() {
    std::cout << "\033[6n";

    // get '\x1b'
    char it = std::cin.get();

    // this while prevents what I call
    // the 'grub' effect - having the cursor
    // moving itself without you pressing a
    // key, because it was too slow to process
    // you input but it was clever enough
    // to remember it
    while (it != '\x1b') {
        it = std::cin.get();
    }

    // get '['
    it = std::cin.get();

    std::string s;
    std::getline(std::cin, s, ';');
    int rows = std::stoi(s);

    std::getline(std::cin, s, 'R');
    int columns = std::stoi(s);

    return std::make_pair(columns - 1, rows - 1);
}


void UnixFeatures::set_cursor(std::pair<int, int> position) {
    auto columns = get_columns();

    if (columns > 0) {
        while (position.first >= columns) {
            position.first -= columns;
            position.second += 1;
        }
    }

    std::cout << "\033[" << std::to_string(position.second + 1) + ';' << std::to_string(position.first + 1) + 'H';
}
