// Copyright (C) 2020 luna_koly
//
// General key codes. Different terminals
// should return consistent character sets
// and here's where we define the key codes.


#pragma once


// raw codes are for internal use only
#ifdef _WIN32
    #define RAW_KEY_RETURN 13
    #define RAW_KEY_BACKSPACE 8
    #define RAW_KEY_UP 'H'
    #define RAW_KEY_DOWN 'P'
    #define RAW_KEY_RIGHT 'M'
    #define RAW_KEY_LEFT 'K'
#else
    #define RAW_KEY_RETURN 10
    #define RAW_KEY_BACKSPACE 127
    #define RAW_KEY_UP 'A'
    #define RAW_KEY_DOWN 'B'
    #define RAW_KEY_RIGHT 'C'
    #define RAW_KEY_LEFT 'D'
#endif
// 27 = '\x1b' = '\033'
#define RAW_KEY_ESCAPE 27
#define RAW_KEY_TAB 9

// terminals 'normalize' their input
// to common symbols or these codes.
// reader then uses them.
#define KEY_RETURN -1
#define KEY_BACKSPACE -2
#define KEY_ESCAPE -3
#define KEY_UP -4
#define KEY_DOWN -5
#define KEY_RIGHT -6
#define KEY_LEFT -7
#define KEY_DELETE -8
#define KEY_TAB -9
