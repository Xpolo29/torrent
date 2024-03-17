#ifndef TRACKER
#define TRACKER

#include "database.h"

#include <netinet/in.h>
#include <sys/socket.h>
#include <string.h>
#include <signal.h>
#include <unistd.h>
#include <stdlib.h>

enum op_t { eq, gt, lt };

int filter(struct data*, char*, long, enum op_t);

int process(int);

#endif
