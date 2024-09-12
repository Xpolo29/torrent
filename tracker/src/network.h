#ifndef NETWORK
#define NETWORK

#include <sys/socket.h>
#include <netinet/in.h>
#include <fcntl.h>
#include <string.h>

//create master listening socket to port <port>.
//returns socket number on success, < 0 otherwise
int create_master_sock(int);

//send <msg> to <socket> return 0 on success, 1 otherwise
int send_msg(int socket, char* msg);

void get_public_ip(char[INET_ADDRSTRLEN]);

int is_local_ip(char[INET_ADDRSTRLEN]);

#endif
