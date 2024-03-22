#ifndef ARGS2
#define ARGS2

#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <ctype.h>

#define LEN_ARGS 10

static const char ARGS[LEN_ARGS][16] = {
	"-v", "--verbose",
       	"-h", "--help",
	"-p", "--port",
    	"-c", "--config",
	"-m", "--max-conn"
};

int load_config(char*);

int apply_parameter(char*, char*);

int parse_args(int, char**);

#endif
