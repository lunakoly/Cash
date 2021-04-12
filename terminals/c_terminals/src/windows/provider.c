#include "../provider.h"

// _isatty
#include <io.h>

bool terminal_is_interactive() {
    return _isatty(0);
}
