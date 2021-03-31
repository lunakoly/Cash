// Copyright (C) 2020 luna_koly
//
// Common code snippets.

#pragma once

/**
 * Constructs a string from the message and
 * an error code.
 */
template <typename T>
const char * report_error(T && message) {
	std::stringstream data;
	data << std::forward<T>(message) << " > errno = " << std::strerror(errno);
	return data.str();
}
