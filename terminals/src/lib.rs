pub mod terminal_stream;

use std::ffi::CStr;
use std::os::raw::c_char;

extern "C" {
    fn terminal_read_line() -> *const c_char;
    fn deallocate_string(pointer: *const c_char);
    fn terminal_has_next() -> bool;
    fn terminal_is_interactive() -> bool;
}

fn replace_and_deallocate(pointer: *const c_char) -> String {
    unsafe {
        let result = CStr::from_ptr(pointer).to_string_lossy().into_owned();
        deallocate_string(pointer);
        return result;
    }
}

fn read_line() -> String {
    unsafe {
        replace_and_deallocate(terminal_read_line())
    }
}

fn has_next() -> bool {
    unsafe {
        terminal_has_next()
    }
}

pub fn is_interactive() -> bool {
    unsafe {
        terminal_is_interactive()
    }
}
