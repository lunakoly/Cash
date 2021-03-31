// Copyright (C) 2020 luna_koly
//
// Windows backend for VT100 platform-dependent
// functionality.


#pragma once


#include "../vt100.hpp"

#include <string>
#include <windows.h>


/**
 * Windows backend for VT100 platform-dependent
 * functionality.
 */
struct WindowsFeatures : public VT100PlatformFeatures {
    __OVERRIDE_ALL_VT100_PLATFORM_FEATURES__

    WindowsFeatures();
    ~WindowsFeatures();

    /**
     * Used for calls to WINAPI.
     */
    HANDLE hOut;
    /**
     * Holds the original terminal configuration.
     * We must restore it when the app exits.
     */
    DWORD original_configuration = 0;
    /**
     * The configuration to use when we need
     * to read user input.
     */
    DWORD raw_configuration = 0;
};
