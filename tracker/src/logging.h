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

static const char* help_message = "Usage: tracker [OPTION...] [OPTION VALUE] \n\n\
	OPTION # OPTION VALUE # DECRIPTION \n\n\
	--verbose or -v # [0:4] (ERROR=0, WARNING (default), LOG, DEBUG, NONE # Sets verbose level \n\
	--help or -h # # Show this message \n\
	--config or -c # <path to config> # Sets path to config.ini \n\
	--max-conn or -m # [1:MAX_TASKS] # Set the number of simultaneous task processing \n\
	--port or -p # [1:65535] # Sets the tracker's listening port \n";


enum LOG_LEVEL { ERROR = 0, WARNING, LOG, DEBUG, NONE };

void logging(enum LOG_LEVEL level, const char* msg, ...);

char *log_level_to_string(enum LOG_LEVEL);

char* get_timestamp();

void print_help();

#endif
