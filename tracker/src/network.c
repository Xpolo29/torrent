#include "network.h"
#include "logging.h"
#include "threads.h"

//create listening socked on port <port>, this one is never closing
int create_master_sock(int port){
	int sock;
	/*
	 * AF_INET = IPV4
	 * SOCK_STREAM = IO_STREAM
	 * IPPROTO_TCP = TCP 
	 */

	sock = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);	
	if(sock < 0){
		logging(ERROR, "Could not open socket\n");	
		return -1;
	}

	int optval = 1;
	if (setsockopt(sock, SOL_SOCKET, SO_REUSEADDR, &optval, sizeof(optval)) < 0) {
		logging(WARNING, "Could not set SO_REUSEADDR flag\n");
	}

	struct sockaddr_in server_addr;

	memset(&server_addr, 0, sizeof(server_addr));
	server_addr.sin_family = AF_INET;
	server_addr.sin_addr.s_addr = INADDR_ANY;
	server_addr.sin_port = htons(port); 

	struct sockaddr* addr = (struct sockaddr*)(&server_addr);
	socklen_t size = sizeof(server_addr);

	int binded = bind(sock, addr, size);

	if(binded < 0){
		logging(ERROR, "Could not bind socket\n");	
		return -1;
	}

	int listened = listen(sock, LEN_TASKS);

	if(listened < 0){
		logging(ERROR, "Could not listen socket\n");	
		return -1;
	}

	fcntl(sock, F_SETFL, O_NONBLOCK);

	return sock;
}


