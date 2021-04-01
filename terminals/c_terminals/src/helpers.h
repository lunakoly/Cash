// Copyright (C) 2020 luna_koly
//
// Common crossplatform code snippets.

#pragma once

/**
 * A pair of 2 integers.
 * Suitable for dimensions and
 * coordinates.
 */
struct PairIntInt {
    int first;
    int second;
};

/**
 * Represents a UTF-8
 * character.
 */
struct Char4 {
    char values[5];
};

/**
 * Constructs a new Char4.
 * Only the first 4 `char`s from
 * the `value` are taken
 */
struct Char4 char4_new(const char * value);
