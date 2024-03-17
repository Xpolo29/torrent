#ifndef LOGGING
#define LOGGING

#include <time.h>
#include <stdarg.h>
#include <dirent.h>
#include <stdio.h>
#include <string.h>
#include <ctype.h>
#include <sys/stat.h>
#include <stdlib.h>

enum LOG_LEVEL { UNSET, ERROR = 0, WARNING, LOG, DEBUG, NONE };

void logging(enum LOG_LEVEL level, const char* msg, ...);

char *log_level_to_string(enum LOG_LEVEL);

char* get_timestamp();

void print_help();

#endif
