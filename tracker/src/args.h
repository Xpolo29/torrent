#ifndef ARGS2
#define ARGS2

#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <ctype.h>

#define LEN_ARGS 12

//list of possible args
static const char ARGS[LEN_ARGS][16] = {
	"-v", "--verbose",
       	"-h", "--help",
	"-p", "--port",
    	"-c", "--config",
	"-m", "--max-conn",
	"-t", "--cache-time"
};

//load config.ini at config_path
int load_config(char*);

//used by load config to apply parameters in file
int apply_parameter(char*, char*);

//used to parse args on cmd
int parse_args(int, char**);

#endif
