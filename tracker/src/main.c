#include "logging.h"
#include "parameters.h"
#include "args.h"
#include "tracker.h"
#include "network.h"

//main fonction, only appear here
int main(int argc, char** argv){

	if (signal(SIGINT, sigint_handler) == SIG_ERR) {
		logging(WARNING, "Cannot catch ctrl+c, exit will be dirty\n");
	}

	//parsing args
	if(parse_args(argc, argv))return 1;

	//loading config
	if(load_config(config_path))return 2;	

	//create thread pool
	if(create_thread_pool(thread_pool_size))return 5;

	//create listening socket
	int main_sock = create_master_sock(port);
	if(main_sock < 0)return 3;

	//init public_ip var
	public_ip[0] = 0;
	get_public_ip(public_ip);


	//connection var
	struct sockaddr_in server_addr;
	struct sockaddr* addr = (struct sockaddr*)(&server_addr);
	memset(&server_addr, 0, sizeof(server_addr));
	socklen_t size = sizeof(server_addr);
	int connection;

	usleep(100000);
	//start
	logging(LOG, "--------------------------------------------------------\n");
	logging(LOG, "Starting on %s at %s|%s:%d\n",
		       	get_timestamp(),
			public_ip,
			private_ip,
			port
	);
	logging(LOG, "--------------------------------------------------------\n");

	//main boucle
	while(running > 0){

		//peer data
		memset(&server_addr, 0, sizeof(server_addr));

		//waiting for connection
		connection = accept(main_sock, addr, &size);

		if(connection > 0){
			//create task to process client request
			new_task(connection);
			mysleep(1);
		} else 
			mysleep(0);
	}

	//clean exit
	close(main_sock);
	if(delete_thread_pool(thread_pool_size)){
		logging(WARNING, "Could not properly delete thread pool");
	}

	//end
	logging(LOG, "--------------------------------------------------------\n");
	logging(LOG, "Tracker stopped at %s\n", get_timestamp());
	logging(LOG, "--------------------------------------------------------\n");
	return 0;
};

