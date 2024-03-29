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

	// Get the local address of the socket
	struct sockaddr_in addr;
	memset(&addr, 0, sizeof(addr));
	socklen_t addr_len = sizeof(addr);
	if (getsockname(connection, (struct sockaddr *)&addr, &addr_len) == -1) {
		logging(WARNING, "Could not fetch ip from socket\n");
	}

	char ip_address[INET_ADDRSTRLEN];
	int port = ntohs(addr.sin_port);
	inet_ntop(AF_INET, &(addr.sin_addr), ip_address, INET_ADDRSTRLEN);

	logging(DEBUG, "Task %d is from %s:%d\n", connection, ip_address, port);
	struct host h = {"", port, time(NULL)};
	strncpy(h.ip, ip_address, 16);

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
