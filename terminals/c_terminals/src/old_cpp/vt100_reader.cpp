// Copyright (C) 2020 luna_koly


#include "vt100_reader.hpp"
#include "keys.hpp"


/**
 * Holds contextual information.
 */
struct Session {
    /**
     * The current ('virtual') line.
     */
    std::vector<char> line;
    /**
     * The caret position within
     * the line.
     */
    int position = 0;
    /**
     * Just a reference to the terminal.
     * Simplifies passing it around.
     */
    VT100Terminal & terminal;

    Session(VT100Terminal & terminal) : terminal(terminal) {
        line.reserve(64);
    }

    /**
     * Returns the final line.
     */
    std::string compose() {
        return std::string(line.begin(), line.end());
    }
};


/**
 * Analyzes the next 'normal' character the user enters.
 */
static void process_character_insert(int it, Session & session) {
    auto position = session.terminal.get_cursor();
    auto width = session.terminal.get_columns();

    // how many characters left in the 'virtual' line
    int tail = (int) session.line.size() - session.position;
    // how many characters left in the 'real' line the cursor is at
    int first_line_end = width - position.first;

    if (session.position >= session.line.size()) {
        session.line.emplace_back(it);
    } else {
        session.line.insert(session.line.begin() + session.position, it);
    }

    session.terminal.put(session.line[session.position]);
    session.terminal.hide_cursor(); // prevents blinking all over the place

    // forces the cursor to move to the next line
    if (width - position.first == 1) {
        session.terminal.put(' ');
        session.terminal.move_left();
    }

    // we're at the right place, so remember that
    position = session.terminal.get_cursor();

    // reprint the line contents
    for (size_t index = session.position + 1; index < session.line.size(); index++) {
        session.terminal.put(session.line[index]);
    }

    // return back to the right place
    session.terminal.set_cursor(position);
    session.terminal.show_cursor();
    session.position += 1;
}


/**
 * Analyzes the next backspace character.
 */
static void process_character_backspace(int it, Session & session) {
    session.terminal.hide_cursor(); // prevents blinking all over the place
    session.terminal.move_left();
    session.position -= 1;

    // we're at the right place, so remember that
    auto position = session.terminal.get_cursor();

    session.line.erase(session.line.begin() + session.position);

    // reprint the line contents
    for (size_t index = session.position; index < session.line.size(); index++) {
        session.terminal.put(session.line[index]);
    }

    // remove last character since
    // everything is moved to the left by 1
    session.terminal.put(' ');
    // return back to the right place
    session.terminal.set_cursor(position);
    session.terminal.show_cursor();
}


/**
 * Analyzes the next character.
 */
static void process_character(int it, Session & session) {
    if (it > 0) {
        process_character_insert(it, session);
    }

    else if (it == KEY_BACKSPACE && session.position > 0) {
        process_character_backspace(it, session);
    }

    else if (it == KEY_RIGHT && session.line.size() > 0 && session.position < session.line.size()) {
        session.terminal.move_right();
        session.position += 1;
    }

    else if (it == KEY_LEFT && session.position > 0) {
        session.terminal.move_left();
        session.position -= 1;
    }
}


std::string VT100Reader::read_line(VT100Terminal & terminal) {
    terminal.to_raw_mode();
    Session session(terminal);

    auto it = terminal.get();

    while (it != KEY_RETURN) {
        process_character(it, session);
        it = terminal.get();
    }

    terminal.to_normal_mode();
    return session.compose();
}
