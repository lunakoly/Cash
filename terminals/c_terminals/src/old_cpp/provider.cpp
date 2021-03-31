// Copyright (C) 2020 luna_koly


#include "provider.hpp"


#ifdef _WIN32
    #include "windows/features.hpp"
    #define FEATURES WindowsFeatures
#else
    #include "unix/features.hpp"
    #define FEATURES UnixFeatures
#endif


static VT100Terminal * terminal = nullptr;


VT100Terminal & TerminalProvider::provide() {
    if (terminal == nullptr) {
        terminal = new VT100Terminal(new FEATURES());
    }

    return *terminal;
}
