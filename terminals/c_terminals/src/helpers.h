// Copyright (C) 2020 luna_koly
//
// Common crossplatform code snippets.

#pragma once

struct PairIntInt {
    int first;
    int second;
};

struct Char4 {
    char values[5];
};

struct Char4 char4_new(const char * value);

// /**
//  * Joins arguments into a single string.
//  */
// std::string join(const std::vector<std::string> & words, const std::string & delimiter = ", ") {
//     std::stringstream line_stream;

//     std::copy(
//         words.begin(),
//         words.end(),
//         std::ostream_iterator<std::string>(line_stream, delimiter.c_str())
//     );

//     return line_stream.str();
// }
