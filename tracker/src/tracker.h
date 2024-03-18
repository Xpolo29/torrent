#ifndef TRACKER
#define TRACKER

#include "database.h"

#include <netinet/in.h>
#include <signal.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

//topmost fonction that is called on each connection,
//handle request parsing, logic processing and answering the peer
int process(int);

//handle ctrl+c for clean exit (thread kill/closing log file/ etc) 
void sigint_handler(int);

#endif
