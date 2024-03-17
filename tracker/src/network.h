#ifndef NETWORK
#define NETWORK

#include <sys/socket.h>
#include <netinet/in.h>
#include <fcntl.h>
#include <string.h>

int create_master_sock(int);

int send_msg(int);

#endif
