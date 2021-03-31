// Copyright (C) 2020 luna_koly
//
// Defines the terminal that supports
// VT100 sequences.


#pragma once


#include <iostream>
#include <string>
#include <vector>
#include <memory>


/**
 * VT100 platform-dependent functionality.
 */
struct VT100PlatformFeatures {
    virtual ~VT100PlatformFeatures() {}

    /**
     * Holds the error message.
     */
    std::string error;

    /**
     * Called before reading user input
     * to configure the terminal properly.
     */
    virtual bool to_raw_mode() = 0;
    /**
     * Called after readign user input
     * to put the terminal back into the
     * normal state.
     */
    virtual bool to_normal_mode() = 0;

    /**
     * Returns the next character.
     */
    virtual int get() = 0;
    /**
     * Returns the number of the columns and
     * the rows.
     * Returned values are in range [0, width - 1].
     */
    virtual std::pair<int, int> get_size() = 0;
    /**
     * Returns the width of the terminal window.
     */
    virtual int get_columns() = 0;
    /**
     * Returns the cursor position.
     * Values must be in range [0, width - 1].
     */
    virtual std::pair<int, int> get_cursor() = 0;
    /**
     * Moves the cursor to the specified position.
     * Values must be in range [0, width - 1].
     */
    virtual void set_cursor(std::pair<int, int>) = 0;
};


/**
 * Just a helper for overriding everything.
 */
#define __OVERRIDE_ALL_VT100_PLATFORM_FEATURES__ \
    virtual bool to_raw_mode() override; \
    virtual bool to_normal_mode() override; \
    virtual int get() override; \
    virtual std::pair<int, int> get_size() override; \
    virtual int get_columns() override; \
    virtual std::pair<int, int> get_cursor() override; \
    virtual void set_cursor(std::pair<int, int>) override; \


/**
 * Terminal with VT100 escape sequences support.
 */
struct VT100Terminal : public VT100PlatformFeatures {
    __OVERRIDE_ALL_VT100_PLATFORM_FEATURES__

    std::unique_ptr<VT100PlatformFeatures> features;

    VT100Terminal(std::unique_ptr<VT100PlatformFeatures> && features)
        : features(std::move(features)) {}

    VT100Terminal(VT100PlatformFeatures * features)
        : VT100Terminal(std::move(std::unique_ptr<VT100PlatformFeatures>(features))) {}

    /**
     * Returns false if an error occured
     * somewhere.
     */
    virtual bool is_ok();
    /**
     * Returns the string describing the
     * error.
     */
    virtual std::string get_error();

    /**
     * Prints the character to the screen and
     * moves the cursor forward by 1. This particular
     * movement step is usually fast.
     */
    virtual void put(char);
    /**
     * Moves the cursor to the left
     * or at the end of the previous line.
     */
    virtual void move_left();
    /**
     * Moves the cursor to the right
     * or at the beginning of the next line.
     */
    virtual void move_right();
    /**
     * Moves the cursor down by the specified
     * number of lines.
     */
    virtual void move_down(int);
    /**
     * Moves the cursor up by the specified
     * number of lines.
     */
    virtual void move_up(int);
    /**
     * Moves the cursor to the specified
     * position on the same line.
     * Position must be in range [0, width - 1].
     */
    virtual void move_directly(int);
    /**
     * Shows the cursor.
     */
    virtual void show_cursor();
    /**
     * Hides the cursor.
     */
    virtual void hide_cursor();

    /**
     * Prompts the user to enter a line.
     * The line is returned when the user hits
     * Enter.
     */
    std::string read_line();
};
