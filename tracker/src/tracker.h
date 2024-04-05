#ifndef TRACKER
#define TRACKER

#define MAX_SLEEPING_TIME 10000

#include "database.h"
#include "parser.h"
#include "network.h"

#include <netinet/in.h>
#include <signal.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>
#include <arpa/inet.h>

//topmost fonction that is called on each connection,
//handle request parsing, logic processing and answering the peer
int process(int);

//handle ctrl+c for clean exit (thread kill/closing log file/ etc) 
void sigint_handler(int);

//return the current waiting time base on workload
void mysleep();

#endif
