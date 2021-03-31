// Copyright (C) 2020 luna_koly


#include "vt100.hpp"

#include <iostream>
#include <string>

#include "vt100_reader.hpp"


bool VT100Terminal::to_raw_mode() {
    return features->to_raw_mode();
}


bool VT100Terminal::to_normal_mode() {
    return features->to_normal_mode();
}


int VT100Terminal::get() {
    return features->get();
}


std::pair<int, int> VT100Terminal::get_size() {
    return features->get_size();
}


int VT100Terminal::get_columns() {
    return features->get_columns();
}


std::pair<int, int> VT100Terminal::get_cursor() {
    return features->get_cursor();
}


void VT100Terminal::set_cursor(std::pair<int, int> position) {
    features->set_cursor(position);
}


bool VT100Terminal::is_ok() {
    return features->error.empty();
}


std::string VT100Terminal::get_error() {
    return features->error;
}


void VT100Terminal::put(char it) {
    std::cout << it;
}


void VT100Terminal::move_left() {
    if (get_cursor().first != 0) {
        std::cout << "\033D";
    } else {
        std::cout << "\033A\033[" + std::to_string(get_columns()) + 'G';
    }
}


void VT100Terminal::move_right() {
    if (get_cursor().first != get_columns() - 1) {
        std::cout << "\033C";
    } else {
        std::cout << "\033B\033[0G";
    }
}


void VT100Terminal::move_down(int count) {
    std::cout << "\033[" + std::to_string(count) + 'B';
}


void VT100Terminal::move_up(int count) {
    std::cout << "\033[" + std::to_string(count) + 'A';
}


void VT100Terminal::move_directly(int position) {
    std::cout << "\033[" + std::to_string(position + 1) + 'G';
}


void VT100Terminal::show_cursor() {
    std::cout << "\033[?25h";
}


void VT100Terminal::hide_cursor() {
    std::cout << "\033[?25l";
}


std::string VT100Terminal::read_line() {
    return VT100Reader::read_line(*this);
}
