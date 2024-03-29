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

//All possible log level
enum LOG_LEVEL { UNSET, ERROR = 0, WARNING, LOG, DEBUG, NONE };

//print and log message if <level> is <= currently set log_level
void logging(enum LOG_LEVEL level, const char* msg, ...);

//transform <log_level> into its string counterpart
char* log_level_to_string(enum LOG_LEVEL);

//return current timestamp formated as "DD-MM-YY@hh:mm:ss"
char* get_timestamp();

//print help in shell
void print_help();

#endif
