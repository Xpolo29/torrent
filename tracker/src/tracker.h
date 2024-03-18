#ifndef TRACKER
#define TRACKER

#include "database.h"

#include <netinet/in.h>
#include <signal.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

int process(int);

void sigint_handler(int);

#endif
