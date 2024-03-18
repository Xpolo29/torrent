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

enum request_t { announce = 0, look, getfile, update };

enum request_t char_to_req(char *request);

char *parse_request(char *buf, char *request, struct host h);

#endif
