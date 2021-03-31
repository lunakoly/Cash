// Copyright (C) 2020 luna_koly
//
// Common code snippets.

#pragma once

#include <string.h>
#include <stdio.h>

/**
 * Constructs a string from the message and
 * an error code.
 */
char * report_error(const char * message) {
	int last_error = (int) GetLastError();
	auto length = snprintf(NULL, 0, "%s > GetLastError() = %d", message, last_error);

	char * result = malloc(length);
	sprintf_s(result, length, "%s > GetLastError() = %d", message, last_error);

	return result;
}
