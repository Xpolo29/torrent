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

int send_msg(int sock, char* msg){
	int len = strlen(msg);
	if(len == 0){
		msg = "Wrong request\n";
		len = strlen(msg);
	}
	logging(LOG, "> %s", msg);
	if (send(sock, msg, len, 0) == -1) {
		logging(WARNING, "Could not answer %d with %s", sock, msg);
		return 1;
	}
	return 0;
}


// get public ip using curl
void get_public_ip(char ip[INET_ADDRSTRLEN]) {
	FILE *fp;
	char path[1024];
	char temp[1024];
	for(int i = 0; i < INET_ADDRSTRLEN; i++)ip[i] = 0;

	// Open the command for reading.
	fp = popen("curl -s icanhazip.com", "r");
	if (fp == NULL) {
	    logging(WARNING, "Failed to fetch public ip from the internet\n");
	    return;
	}
	while (fgets(path, sizeof(path), fp) != NULL) {
		strcpy(temp, path);
	}

	int i = 0;
	for(; i < INET_ADDRSTRLEN; i++){
		if(temp[i] != '\n')
			ip[i] = temp[i];
	}
	ip[i] = 0;

	// close
	pclose(fp);
	if(strlen(ip) == 0){
		strcpy(ip, "");
		logging(WARNING, "Could not fecth public ip, local mode only\n");
	}
	logging(DEBUG, "Public ip is %s\n", ip);
}

// check if ip is local
int is_local_ip(char ip_address[INET_ADDRSTRLEN]){
	char beginning[4];
	memcpy(beginning, ip_address, 3);
	beginning[3] = 0;
	
	int b1 = !strcmp(beginning, "192"); 
	int b2 = !strcmp(beginning, "10."); 

	return b1 || b2;
}
