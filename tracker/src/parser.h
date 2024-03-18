#ifndef PARSER
#define PARSER

#include "database.h"
#include <regex.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define MATCH_SIZE 10
#define MAX_SEED 32
#define HASH_SIZE 64

//type of tcp incoming message
enum request_t { announce = 0, look, getfile, update };

//idk
enum request_t char_to_req(char *request);

//idk
int parse_request(char *buf, char *request, struct host h);

#endif
