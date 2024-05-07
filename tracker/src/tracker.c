#include "tracker.h"
#include "logging.h"
#include "parameters.h"

// Catch ctrl+c for clean exit
void sigint_handler(int signum) {
  if (signum != SIGINT)
    return;
  logging(LOG, "Ctrl+c received, exiting\n");
  running--;
  if (running < -1) {
    logging(WARNING, "Double ctrl+c received, forcing exit\n");
    exit(6);
  }
}

void mysleep(){
	if(task_len == 0)usleep(MAX_SLEEPING_TIME);
}

// Handle request comprehension and answers for peer <connection>
int process(int connection) {
	char buff[16 * 1024] = {0};
	fcntl(connection, F_SETFL, O_NONBLOCK);
	int read = recv(connection, buff, 1024 * 16, 0);

	if (read < 0) {
		mysleep();
		new_task(connection);

		return 0;
	}

	logging(LOG, "< %s", buff);

	// Init local address if uninitialised
	if(strlen(private_ip) == 0){
		struct sockaddr_in localAddress;
		socklen_t addressLength = sizeof(localAddress);
		if (getsockname(connection, (struct sockaddr *)&localAddress, &addressLength) == -1) {
			logging(ERROR, "Error getting local address\n");
		}

		char ipStr[INET_ADDRSTRLEN];
		inet_ntop(AF_INET, &(localAddress.sin_addr), ipStr, INET_ADDRSTRLEN);
		logging(LOG, "Initialising local ip to : %s\n", ipStr);
		strcpy(private_ip, ipStr);
		if(strlen(public_ip) == 0)strcpy(public_ip, private_ip);
	}

	// Get the remote address of the socket
	struct sockaddr_in addr;
	memset(&addr, 0, sizeof(addr));
	socklen_t addr_len = sizeof(addr);
	if (getpeername(connection, (struct sockaddr *)&addr, &addr_len) == -1) {
		logging(WARNING, "Could not fetch remote ip from socket\n");
	}

	// retrieve connection ip and port
	char ip_address[INET_ADDRSTRLEN];

	inet_ntop(AF_INET, &(addr.sin_addr), ip_address, INET_ADDRSTRLEN);
	uint16_t port = ntohs(addr.sin_port);


	// get port from db is exists
	struct host hosts[BDD_SIZE]; 
	int len = load_ip(hosts, ip_address);
	if(len == 1)port = hosts[0].port;

	struct host h = {"", port, time(NULL)};
	strncpy(h.ip, ip_address, INET_ADDRSTRLEN);

	logging(DEBUG, "Task %d is from %s:%hu\n", connection, h.ip, h.port);

	/*
	mimic worload
	sleep(1);
	*/

	//cleanup ttl
	remove_old_entries();

	// parsing request
	char out[16 * 1024];
	memset(out, 0, 16*1024);
	parse_request(out, buff, h);

	// answer peer
	send_msg(connection, out);

	close(connection);
	return 0;
}
