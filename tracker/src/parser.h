#ifndef PARSER
#define PARSER

#include "database.h"
#include "network.h"
#include "parameters.h"
#include "database.h"
#include "logging.h"

#include <regex.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <string.h>
#include <strings.h>

#define MATCH_SIZE 10
#define MAX_SEED 32
#define HASH_SIZE 64

//type of tcp incoming message
enum request_t { announce = 0, look, getfile, update };

//convert string to enum request_t
enum request_t char_to_req(char *request);

//i
int parse_request(char *buf, char *request, struct host h);

#endif
