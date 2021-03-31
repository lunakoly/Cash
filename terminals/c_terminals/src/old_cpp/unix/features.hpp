// Copyright (C) 2020 luna_koly
//
// Unix backend for VT100 platform-dependent
// functionality.


#pragma once


#include "../vt100.hpp"


#include <string>
#include <termios.h>


/**
 * Unix backend for VT100 platform-dependent
 * functionality.
 */
struct UnixFeatures : public VT100PlatformFeatures {
    __OVERRIDE_ALL_VT100_PLATFORM_FEATURES__

    UnixFeatures();
    virtual ~UnixFeatures() override;

    /**
     * Holds the original terminal configuration.
     * We must restore it when the app exits.
     */
    termios original_configuration;
    /**
     * The configuration to use when we need
     * to read user input.
     */
    termios raw_configuration;
};
